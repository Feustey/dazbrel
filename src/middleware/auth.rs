use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use subtle::ConstantTimeEq;
use tracing::warn;

type HmacSha256 = Hmac<Sha256>;

lazy_static::lazy_static! {
    static ref SECRET_KEY: Vec<u8> = load_secret_key();
    static ref TOKEN_TTL_SECONDS: u64 = load_token_ttl_seconds();
}

fn load_secret_key() -> Vec<u8> {
    if let Ok(secret_file) = std::env::var("AUTH_SECRET_KEY_FILE") {
        if let Ok(contents) = std::fs::read_to_string(secret_file) {
            let trimmed = contents.trim();
            if !trimmed.is_empty() {
                return trimmed.as_bytes().to_vec();
            }
        }
    }

    if let Ok(secret) = std::env::var("AUTH_SECRET_KEY") {
        let trimmed = secret.trim();
        if !trimmed.is_empty() {
            return trimmed.as_bytes().to_vec();
        }
    }

    warn!("AUTH_SECRET_KEY not configured; using insecure fallback for development only.");
    b"dazno-secret-key-should-be-in-env".to_vec()
}

fn load_token_ttl_seconds() -> u64 {
    std::env::var("AUTH_TOKEN_TTL_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(3600)
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
    InvalidFormat,
}

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extraire le token d'authentification
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => {
            warn!("Missing authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if !auth_header.starts_with("Bearer ") {
        warn!("Invalid authorization header format");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Retirer "Bearer "

    if let Err(auth_error) = validate_token(token) {
        warn!("Authentication failed: {:?}", auth_error);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Token valide, continuer
    Ok(next.run(request).await)
}

fn validate_token(token: &str) -> Result<(), AuthError> {
    // Décoder le token base64
    let decoded = general_purpose::STANDARD
        .decode(token)
        .map_err(|_| AuthError::InvalidFormat)?;

    let token_str = String::from_utf8(decoded).map_err(|_| AuthError::InvalidFormat)?;

    // Format attendu: "timestamp:signature"
    let parts: Vec<&str> = token_str.split(':').collect();
    if parts.len() != 2 {
        return Err(AuthError::InvalidFormat);
    }

    let timestamp_str = parts[0];
    let provided_signature = parts[1];

    // Vérifier que le timestamp n'est pas expiré
    let timestamp: u64 = timestamp_str
        .parse()
        .map_err(|_| AuthError::InvalidFormat)?;

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Token expire après TTL configuré
    if current_timestamp.saturating_sub(timestamp) > *TOKEN_TTL_SECONDS {
        return Err(AuthError::ExpiredToken);
    }

    // Calculer la signature attendue
    let mut mac = HmacSha256::new_from_slice(&SECRET_KEY).map_err(|_| AuthError::InvalidToken)?;
    mac.update(timestamp_str.as_bytes());

    let expected_signature = mac.finalize().into_bytes();
    let provided_signature =
        hex::decode(provided_signature).map_err(|_| AuthError::InvalidFormat)?;

    if provided_signature.len() != expected_signature.len() {
        return Err(AuthError::InvalidToken);
    }

    // Comparaison constante pour éviter les attaques par timing
    if provided_signature
        .ct_eq(expected_signature.as_slice())
        .into()
    {
        Ok(())
    } else {
        Err(AuthError::InvalidToken)
    }
}

/// Génère un token d'authentification valide pour les tests
pub fn generate_auth_token() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut mac = HmacSha256::new_from_slice(&SECRET_KEY).unwrap();
    mac.update(timestamp.to_string().as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    let token_data = format!("{}:{}", timestamp, signature);
    general_purpose::STANDARD.encode(token_data)
}

/// Middleware pour les routes publiques (santé, statut)
pub async fn public_route_middleware(request: Request, next: Next) -> Response {
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let token = generate_auth_token();
        assert!(validate_token(&token).is_ok());
    }

    #[test]
    fn test_invalid_token_format() {
        assert!(matches!(
            validate_token("invalid"),
            Err(AuthError::InvalidFormat)
        ));
        assert!(matches!(validate_token(""), Err(AuthError::InvalidFormat)));
    }

    #[test]
    fn test_expired_token() {
        // Créer un token avec un timestamp très ancien
        let old_timestamp = 1000000; // Timestamp très ancien
        let mut mac = HmacSha256::new_from_slice(&SECRET_KEY).unwrap();
        mac.update(old_timestamp.to_string().as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let token_data = format!("{}:{}", old_timestamp, signature);
        let token = general_purpose::STANDARD.encode(token_data);

        assert!(matches!(
            validate_token(&token),
            Err(AuthError::ExpiredToken)
        ));
    }
}
