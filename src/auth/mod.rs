use anyhow::{Context, Result};
use argon2::password_hash::{rand_core::OsRng, Error as PasswordHashError, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::{info, warn};
use uuid::Uuid;

pub mod session;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
    pub must_change_password: bool,
}

/// Service d'authentification avec Argon2
pub struct AuthService {
    db: SqlitePool,
}

impl AuthService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    /// Crée les tables nécessaires pour l'authentification
    pub async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_login TEXT,
                is_admin BOOLEAN NOT NULL DEFAULT 0,
                must_change_password BOOLEAN NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                ip_address TEXT,
                user_agent TEXT,
                FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.db)
        .await?;

        info!("Tables d'authentification créées");
        Ok(())
    }

    /// Initialise l'utilisateur par défaut avec un mot de passe généré
    pub async fn initialize_default_user(&self) -> Result<String> {
        // Vérifier si un utilisateur existe déjà
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.db)
            .await?;

        if user_count > 0 {
            info!("Utilisateur par défaut déjà existant");
            return Ok("Utilisateur existant".to_string());
        }

        // Générer un mot de passe sécurisé par défaut
        let default_password = self.generate_secure_password();

        // Créer l'utilisateur admin par défaut
        let user_id = Uuid::new_v4();
        let password_hash = self.hash_password(&default_password)?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, created_at, is_admin, must_change_password)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#
        )
        .bind(user_id.to_string())
        .bind("admin")
        .bind(password_hash)
        .bind(Utc::now().to_rfc3339())
        .bind(true)
        .bind(true)
        .execute(&self.db)
        .await?;

        info!("Utilisateur admin par défaut créé avec succès");
        Ok(default_password)
    }

    /// Génère un mot de passe sécurisé
    fn generate_secure_password(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        let mut rng = rand::thread_rng();

        (0..18)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Hash un mot de passe avec Argon2
    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Erreur de hashage: {:?}", e))?
            .to_string();
        Ok(password_hash)
    }

    /// Vérifie un mot de passe contre son hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Erreur de parsing du hash: {:?}", e))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Authentifie un utilisateur
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT id, username, password_hash, created_at, last_login, is_admin, must_change_password
            FROM users 
            WHERE username = ?1
            "#
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
                    .with_timezone(&Utc),
                last_login: row
                    .get::<Option<String>, _>("last_login")
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                is_admin: row.get::<bool, _>("is_admin"),
                must_change_password: row.get::<bool, _>("must_change_password"),
            };

            if self.verify_password(password, &user.password_hash)? {
                // Mettre à jour la dernière connexion
                sqlx::query("UPDATE users SET last_login = ?1 WHERE id = ?2")
                    .bind(Utc::now().to_rfc3339())
                    .bind(user.id.to_string())
                    .execute(&self.db)
                    .await?;

                info!("Authentification réussie pour l'utilisateur: {}", username);
                return Ok(Some(user));
            } else {
                warn!(
                    "Tentative d'authentification avec mot de passe incorrect pour: {}",
                    username
                );
            }
        } else {
            warn!(
                "Tentative d'authentification avec utilisateur inexistant: {}",
                username
            );
        }

        Ok(None)
    }

    /// Change le mot de passe d'un utilisateur
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<bool> {
        // Récupérer l'utilisateur actuel
        let row = sqlx::query(
            "SELECT id, username, password_hash, created_at, last_login, is_admin, must_change_password FROM users WHERE id = ?1"
        )
        .bind(user_id.to_string())
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
                    .with_timezone(&Utc),
                last_login: row
                    .get::<Option<String>, _>("last_login")
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                is_admin: row.get::<bool, _>("is_admin"),
                must_change_password: row.get::<bool, _>("must_change_password"),
            };

            // Vérifier l'ancien mot de passe
            if !self.verify_password(current_password, &user.password_hash)? {
                warn!("Tentative de changement de mot de passe avec ancien mot de passe incorrect");
                return Ok(false);
            }

            // Valider le nouveau mot de passe
            if !self.is_password_strong(new_password) {
                return Err(anyhow::anyhow!(
                    "Le nouveau mot de passe ne respecte pas les critères de sécurité"
                ));
            }

            // Hasher le nouveau mot de passe
            let new_hash = self.hash_password(new_password)?;

            // Mettre à jour en base
            sqlx::query(
                "UPDATE users SET password_hash = ?1, must_change_password = ?2 WHERE id = ?3",
            )
            .bind(new_hash)
            .bind(false) // Plus besoin de changer le mot de passe
            .bind(user_id.to_string())
            .execute(&self.db)
            .await?;

            info!(
                "Mot de passe changé avec succès pour l'utilisateur: {}",
                user.username
            );
            return Ok(true);
        }

        Ok(false)
    }

    /// Valide la force d'un mot de passe
    fn is_password_strong(&self, password: &str) -> bool {
        password.len() >= 8
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_numeric())
            && password.chars().any(|c| !c.is_alphanumeric())
    }

    /// Récupère un utilisateur par ID
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, created_at, last_login, is_admin, must_change_password FROM users WHERE id = ?1"
        )
        .bind(user_id.to_string())
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?
                    .with_timezone(&Utc),
                last_login: row
                    .get::<Option<String>, _>("last_login")
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                is_admin: row.get::<bool, _>("is_admin"),
                must_change_password: row.get::<bool, _>("must_change_password"),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}
