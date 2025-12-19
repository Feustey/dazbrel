use anyhow::Result;
use sqlx::SqlitePool;
use std::time::Duration;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

/// Configuration des sessions pour l'authentification
pub struct SessionConfig {
    pub secret_key: String,
    pub max_age: Duration,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: tower_sessions::cookie::SameSite,
}

/// Crée une couche de session en mémoire pour le développement
pub fn create_memory_session_layer(config: SessionConfig) -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::default();
    SessionManagerLayer::new(store)
        .with_secure(config.secure)
        .with_http_only(config.http_only)
        .with_same_site(config.same_site)
}

/// Crée une couche de session SQLite pour la production
pub async fn create_sqlite_session_layer(
    db_pool: SqlitePool,
    config: SessionConfig,
) -> Result<SessionManagerLayer<SqliteStore>> {
    let store = SqliteStore::new(db_pool);
    store.migrate().await?;

    Ok(SessionManagerLayer::new(store)
        .with_secure(config.secure)
        .with_http_only(config.http_only)
        .with_same_site(config.same_site))
}

/// Configuration de session pour le développement
pub fn development_session_config() -> SessionConfig {
    SessionConfig {
        secret_key: "your-secret-key-change-in-production".to_string(),
        max_age: Duration::from_secs(24 * 60 * 60), // 24 heures
        secure: false,                              // HTTP autorisé en développement
        http_only: true,
        same_site: tower_sessions::cookie::SameSite::Lax,
    }
}

/// Configuration de session pour la production
pub fn production_session_config() -> SessionConfig {
    SessionConfig {
        secret_key: std::env::var("SESSION_SECRET_KEY")
            .unwrap_or_else(|_| "change-this-in-production".to_string()),
        max_age: Duration::from_secs(2 * 60 * 60), // 2 heures
        secure: true,                              // HTTPS uniquement en production
        http_only: true,
        same_site: tower_sessions::cookie::SameSite::Strict,
    }
}
