use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tracing::{debug, warn};

#[derive(Debug, Clone)]
pub struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_duration: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100, // 100 requêtes par minute par défaut
            window_duration: Duration::from_secs(60),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitState {
    entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
}

impl RateLimitState {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub fn check_rate_limit(&self, client_key: &str) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();

        match entries.get_mut(client_key) {
            Some(entry) => {
                // Vérifier si la fenêtre de temps est expirée
                if now.duration_since(entry.window_start) >= self.config.window_duration {
                    // Nouvelle fenêtre
                    entry.count = 1;
                    entry.window_start = now;
                    true
                } else {
                    // Dans la même fenêtre
                    if entry.count >= self.config.max_requests {
                        false // Rate limit dépassé
                    } else {
                        entry.count += 1;
                        true
                    }
                }
            }
            None => {
                // Première requête pour ce client
                entries.insert(
                    client_key.to_string(),
                    RateLimitEntry {
                        count: 1,
                        window_start: now,
                    },
                );
                true
            }
        }
    }

    pub fn cleanup_expired_entries(&self) {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();

        entries.retain(|_, entry| {
            now.duration_since(entry.window_start) < self.config.window_duration * 2
        });
    }
}

impl Default for RateLimitState {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Utiliser l'adresse IP comme clé de rate limiting
    let client_key = addr.ip().to_string();

    // Pour cette implémentation, on crée un state temporaire
    // Dans une vraie application, cela devrait être partagé via l'AppState
    thread_local! {
        static RATE_LIMITER: RateLimitState = RateLimitState::default();
    }

    let allowed = RATE_LIMITER.with(|limiter| limiter.check_rate_limit(&client_key));

    if !allowed {
        warn!("Rate limit exceeded for client: {}", client_key);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    debug!("Rate limit check passed for client: {}", client_key);
    Ok(next.run(request).await)
}

/// Version plus flexible qui prend le state en paramètre
pub fn create_rate_limit_middleware(
    state: RateLimitState,
) -> impl Fn(
    ConnectInfo<SocketAddr>,
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
> + Clone {
    move |ConnectInfo(addr): ConnectInfo<SocketAddr>, request: Request, next: Next| {
        let state = state.clone();
        Box::pin(async move {
            let client_key = addr.ip().to_string();

            if !state.check_rate_limit(&client_key) {
                warn!("Rate limit exceeded for client: {}", client_key);
                return Err(StatusCode::TOO_MANY_REQUESTS);
            }

            debug!("Rate limit check passed for client: {}", client_key);
            Ok(next.run(request).await)
        })
    }
}

/// Configuration spéciale pour les endpoints critiques
pub fn create_strict_rate_limiter() -> RateLimitState {
    RateLimitState::new(RateLimitConfig {
        max_requests: 10, // Seulement 10 requêtes par minute
        window_duration: Duration::from_secs(60),
    })
}

/// Configuration pour les endpoints d'exécution d'actions
pub fn create_action_rate_limiter() -> RateLimitState {
    RateLimitState::new(RateLimitConfig {
        max_requests: 5, // Très restrictif pour les actions financières
        window_duration: Duration::from_secs(300), // 5 minutes
    })
}

/// Task de nettoyage pour supprimer les entrées expirées
pub async fn cleanup_rate_limit_entries(state: RateLimitState) {
    let mut interval = tokio::time::interval(Duration::from_secs(300)); // Nettoyer toutes les 5 minutes

    loop {
        interval.tick().await;
        state.cleanup_expired_entries();
        debug!("Rate limit entries cleanup completed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_rate_limit_within_limit() {
        let config = RateLimitConfig {
            max_requests: 5,
            window_duration: Duration::from_secs(60),
        };
        let state = RateLimitState::new(config);

        // Les 5 premières requêtes doivent passer
        for i in 1..=5 {
            assert!(
                state.check_rate_limit("test_client"),
                "Request {} should pass",
                i
            );
        }
    }

    #[test]
    fn test_rate_limit_exceeded() {
        let config = RateLimitConfig {
            max_requests: 3,
            window_duration: Duration::from_secs(60),
        };
        let state = RateLimitState::new(config);

        // Les 3 premières requêtes passent
        for i in 1..=3 {
            assert!(
                state.check_rate_limit("test_client"),
                "Request {} should pass",
                i
            );
        }

        // La 4ème doit être bloquée
        assert!(
            !state.check_rate_limit("test_client"),
            "Request 4 should be blocked"
        );
    }

    #[test]
    fn test_rate_limit_window_reset() {
        let config = RateLimitConfig {
            max_requests: 2,
            window_duration: Duration::from_millis(100),
        };
        let state = RateLimitState::new(config);

        // Atteindre la limite
        assert!(state.check_rate_limit("test_client"));
        assert!(state.check_rate_limit("test_client"));
        assert!(!state.check_rate_limit("test_client"));

        // Attendre que la fenêtre expire
        thread::sleep(Duration::from_millis(150));

        // Maintenant ça devrait marcher à nouveau
        assert!(state.check_rate_limit("test_client"));
    }

    #[test]
    fn test_different_clients() {
        let config = RateLimitConfig {
            max_requests: 2,
            window_duration: Duration::from_secs(60),
        };
        let state = RateLimitState::new(config);

        // Client 1 atteint sa limite
        assert!(state.check_rate_limit("client1"));
        assert!(state.check_rate_limit("client1"));
        assert!(!state.check_rate_limit("client1"));

        // Client 2 devrait pouvoir faire des requêtes
        assert!(state.check_rate_limit("client2"));
        assert!(state.check_rate_limit("client2"));
        assert!(!state.check_rate_limit("client2"));
    }

    #[test]
    fn test_cleanup_expired_entries() {
        let config = RateLimitConfig {
            max_requests: 5,
            window_duration: Duration::from_millis(50),
        };
        let state = RateLimitState::new(config);

        // Ajouter quelques entrées
        state.check_rate_limit("client1");
        state.check_rate_limit("client2");

        // Vérifier qu'elles existent
        {
            let entries = state.entries.lock().unwrap();
            assert_eq!(entries.len(), 2);
        }

        // Attendre que les entrées expirent
        thread::sleep(Duration::from_millis(200));

        // Nettoyer
        state.cleanup_expired_entries();

        // Vérifier que les entrées ont été supprimées
        {
            let entries = state.entries.lock().unwrap();
            assert_eq!(entries.len(), 0);
        }
    }
}
