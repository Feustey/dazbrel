use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputValidationError {
    InvalidFormat,
    TooLong,
    TooShort,
    InvalidCharacters,
    InvalidRange,
    SqlInjection,
    XssAttempt,
}

#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<Regex>,
    pub allowed_chars: Option<Regex>,
    pub numeric_range: Option<(f64, f64)>,
}

impl Default for ValidationRule {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: Some(1000), // Limite par défaut
            pattern: None,
            allowed_chars: None,
            numeric_range: None,
        }
    }
}

pub struct InputValidator {
    rules: HashMap<String, ValidationRule>,
}

impl Default for InputValidator {
    fn default() -> Self {
        let mut rules = HashMap::new();

        // Règles pour les IDs de recommandation
        rules.insert(
            "recommendation_id".to_string(),
            ValidationRule {
                min_length: Some(1),
                max_length: Some(100),
                pattern: Some(Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap()),
                ..Default::default()
            },
        );

        // Règles pour les clés publiques Bitcoin (66 ou 68 chars selon format)
        rules.insert(
            "pubkey".to_string(),
            ValidationRule {
                min_length: Some(66),
                max_length: Some(68),
                pattern: Some(Regex::new(r"^[0-9a-fA-F]{66,68}$").unwrap()),
                ..Default::default()
            },
        );

        // Règles pour les IDs de canal
        rules.insert(
            "channel_id".to_string(),
            ValidationRule {
                min_length: Some(1),
                max_length: Some(50),
                pattern: Some(Regex::new(r"^[0-9]+$").unwrap()),
                ..Default::default()
            },
        );

        // Règles pour les montants (satoshis)
        rules.insert(
            "amount".to_string(),
            ValidationRule {
                numeric_range: Some((1.0, 100_000_000.0)), // 1 sat à 1 BTC
                ..Default::default()
            },
        );

        // Règles pour les taux de frais
        rules.insert(
            "fee_rate".to_string(),
            ValidationRule {
                numeric_range: Some((0.0, 10000.0)), // 0 à 10000 ppm
                ..Default::default()
            },
        );

        // Règles pour les messages/descriptions
        rules.insert(
            "message".to_string(),
            ValidationRule {
                max_length: Some(500),
                allowed_chars: Some(Regex::new(r"^[a-zA-Z0-9\s\.,!?\-_()]+$").unwrap()),
                ..Default::default()
            },
        );

        Self { rules }
    }
}

impl InputValidator {
    pub fn validate(&self, field_name: &str, value: &str) -> Result<(), InputValidationError> {
        // Vérifications générales de sécurité
        self.check_security_threats(value)?;

        // Appliquer les règles spécifiques au champ
        if let Some(rule) = self.rules.get(field_name) {
            self.apply_rule(value, rule)?;
        }

        Ok(())
    }

    pub fn validate_numeric(
        &self,
        field_name: &str,
        value: f64,
    ) -> Result<(), InputValidationError> {
        if let Some(rule) = self.rules.get(field_name) {
            if let Some((min, max)) = rule.numeric_range {
                if value < min || value > max {
                    return Err(InputValidationError::InvalidRange);
                }
            }
        }
        Ok(())
    }

    fn check_security_threats(&self, value: &str) -> Result<(), InputValidationError> {
        // Détecter les tentatives d'injection SQL
        let sql_patterns = [
            "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "CREATE", "ALTER", "EXEC", "UNION",
            "SCRIPT", "--", "/*", "*/",
        ];

        let value_upper = value.to_uppercase();
        for pattern in &sql_patterns {
            if value_upper.contains(pattern) {
                warn!("Potential SQL injection attempt detected: {}", value);
                return Err(InputValidationError::SqlInjection);
            }
        }

        // Détecter les tentatives XSS
        let xss_patterns = [
            "<script",
            "</script>",
            "javascript:",
            "onload=",
            "onerror=",
            "onclick=",
        ];
        let value_lower = value.to_lowercase();
        for pattern in &xss_patterns {
            if value_lower.contains(pattern) {
                warn!("Potential XSS attempt detected: {}", value);
                return Err(InputValidationError::XssAttempt);
            }
        }

        Ok(())
    }

    fn apply_rule(&self, value: &str, rule: &ValidationRule) -> Result<(), InputValidationError> {
        // Vérifier la longueur minimale
        if let Some(min_len) = rule.min_length {
            if value.len() < min_len {
                return Err(InputValidationError::TooShort);
            }
        }

        // Vérifier la longueur maximale
        if let Some(max_len) = rule.max_length {
            if value.len() > max_len {
                return Err(InputValidationError::TooLong);
            }
        }

        // Vérifier le pattern
        if let Some(pattern) = &rule.pattern {
            if !pattern.is_match(value) {
                return Err(InputValidationError::InvalidFormat);
            }
        }

        // Vérifier les caractères autorisés
        if let Some(allowed) = &rule.allowed_chars {
            if !allowed.is_match(value) {
                return Err(InputValidationError::InvalidCharacters);
            }
        }

        Ok(())
    }
}

// Instance globale du validateur
lazy_static::lazy_static! {
    static ref VALIDATOR: InputValidator = InputValidator::default();
}

pub fn validate_input(field_name: &str, value: &str) -> Result<(), InputValidationError> {
    VALIDATOR.validate(field_name, value)
}

pub fn validate_numeric_input(field_name: &str, value: f64) -> Result<(), InputValidationError> {
    VALIDATOR.validate_numeric(field_name, value)
}

/// Middleware pour valider automatiquement les entrées dans les requêtes
pub async fn validation_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    // Pour une validation plus approfondie, on pourrait inspecter le body ici
    // Pour l'instant, on laisse la validation aux handlers individuels
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_recommendation_id() {
        assert!(validate_input("recommendation_id", "rec_12345").is_ok());
        assert!(validate_input("recommendation_id", "recommendation-abc").is_ok());
    }

    #[test]
    fn test_invalid_recommendation_id() {
        assert!(validate_input("recommendation_id", "").is_err());
        assert!(validate_input("recommendation_id", "rec with spaces").is_err());
        assert!(validate_input("recommendation_id", "rec@#$%").is_err());
    }

    #[test]
    fn test_valid_pubkey() {
        let valid_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        assert!(validate_input("pubkey", valid_pubkey).is_ok());
    }

    #[test]
    fn test_invalid_pubkey() {
        assert!(validate_input("pubkey", "short").is_err());
        assert!(validate_input(
            "pubkey",
            "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdefff"
        )
        .is_err()); // Too long
        assert!(validate_input(
            "pubkey",
            "invalid_characters_in_pubkey_!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
        )
        .is_err());
    }

    #[test]
    fn test_sql_injection_detection() {
        assert!(validate_input("message", "'; DROP TABLE users; --").is_err());
        assert!(validate_input("message", "SELECT * FROM passwords").is_err());
        assert!(validate_input("message", "UNION SELECT password FROM users").is_err());
    }

    #[test]
    fn test_xss_detection() {
        assert!(validate_input("message", "<script>alert('xss')</script>").is_err());
        assert!(validate_input("message", "javascript:alert('xss')").is_err());
        assert!(validate_input("message", "<img onerror=alert('xss')>").is_err());
    }

    #[test]
    fn test_numeric_validation() {
        assert!(validate_numeric_input("amount", 1000.0).is_ok());
        assert!(validate_numeric_input("amount", 0.5).is_err()); // Too small
        assert!(validate_numeric_input("amount", 200_000_000.0).is_err()); // Too large

        assert!(validate_numeric_input("fee_rate", 500.0).is_ok());
        assert!(validate_numeric_input("fee_rate", -100.0).is_err()); // Negative
        assert!(validate_numeric_input("fee_rate", 15000.0).is_err()); // Too high
    }

    #[test]
    fn test_channel_id_validation() {
        assert!(validate_input("channel_id", "123456789").is_ok());
        assert!(validate_input("channel_id", "not_numeric").is_err());
        assert!(validate_input("channel_id", "").is_err());
    }
}
