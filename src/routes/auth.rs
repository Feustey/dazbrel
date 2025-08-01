use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_sessions::Session;
use tracing::{error, info, warn};
use handlebars::Handlebars;
use uuid::Uuid;

use crate::auth::{AuthService, User};

#[derive(Deserialize)]
pub struct LoginQuery {
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Serialize)]
pub struct AuthStatusResponse {
    pub authenticated: bool,
    pub username: Option<String>,
    pub must_change_password: bool,
    pub is_admin: bool,
}

/// Page de connexion
pub async fn login_page(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
    Query(query): Query<LoginQuery>,
    session: Session,
) -> Result<Html<String>, StatusCode> {
    // Vérifier si l'utilisateur est déjà connecté
    if let Ok(Some(_)) = get_current_user(&app_state.auth_service, &session).await {
        return Ok(Html(
            "<script>window.location.href='/';</script>".to_string()
        ));
    }

    let context = json!({
        "is_first_login": false,
        "default_password": "",
        "error": query.error
    });

    let html = app_state.handlebars
        .render("login", &context)
        .map_err(|e| {
            error!("Erreur de rendu du template login: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html))
}

/// Traitement de la connexion
pub async fn login_post(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
    session: Session,
    Form(login_request): Form<LoginRequest>,
) -> impl IntoResponse {
    match app_state.auth_service.authenticate(&login_request.username, &login_request.password).await {
        Ok(Some(user)) => {
            // Connexion réussie
            if let Err(e) = login_user(&session, &user).await {
                error!("Erreur lors de la création de session: {}", e);
                return render_login_error(&app_state.handlebars, "Erreur interne lors de la connexion");
            }

            info!("Utilisateur connecté: {}", user.username);
            
            // Rediriger vers la page de changement de mot de passe si nécessaire
            if user.must_change_password {
                Redirect::to("/change-password").into_response()
            } else {
                Redirect::to("/").into_response()
            }
        }
        Ok(None) => {
            warn!("Tentative de connexion échouée pour: {}", login_request.username);
            render_login_error(&app_state.handlebars, "Nom d'utilisateur ou mot de passe incorrect")
        }
        Err(e) => {
            error!("Erreur lors de l'authentification: {}", e);
            render_login_error(&app_state.handlebars, "Erreur interne lors de la connexion")
        }
    }
}

/// Page de changement de mot de passe
pub async fn change_password_page(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
    session: Session,
) -> Result<impl IntoResponse, StatusCode> {
    // Vérifier que l'utilisateur est connecté
    let user = match get_current_user(&app_state.auth_service, &session).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Redirect::to("/login").into_response()),
        Err(e) => {
            error!("Erreur lors de la vérification d'authentification: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let context = json!({
        "username": user.username,
        "must_change_password": user.must_change_password
    });

    let html = app_state.handlebars
        .render("change-password", &context)
        .map_err(|e| {
            error!("Erreur de rendu du template change-password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html).into_response())
}

/// Traitement du changement de mot de passe
pub async fn change_password_post(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
    session: Session,
    Form(change_request): Form<ChangePasswordRequest>,
) -> impl IntoResponse {
    // Vérifier que l'utilisateur est connecté
    let user = match get_current_user(&app_state.auth_service, &session).await {
        Ok(Some(user)) => user,
        Ok(None) => return Redirect::to("/login").into_response(),
        Err(e) => {
            error!("Erreur lors de la vérification d'authentification: {}", e);
            return render_change_password_error(&app_state.handlebars, "Erreur interne");
        }
    };

    // Valider que les nouveaux mots de passe correspondent
    if change_request.new_password != change_request.confirm_password {
        return render_change_password_error(&app_state.handlebars, "Les nouveaux mots de passe ne correspondent pas");
    }

    match app_state.auth_service.change_password(
        user.id,
        &change_request.current_password,
        &change_request.new_password,
    ).await {
        Ok(true) => {
            info!("Mot de passe changé avec succès pour: {}", user.username);
            Redirect::to("/?password_changed=true").into_response()
        }
        Ok(false) => {
            warn!("Échec du changement de mot de passe pour: {}", user.username);
            render_change_password_error(&app_state.handlebars, "Mot de passe actuel incorrect")
        }
        Err(e) => {
            error!("Erreur lors du changement de mot de passe: {}", e);
            render_change_password_error(&app_state.handlebars, &e.to_string())
        }
    }
}

/// Déconnexion
pub async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    info!("Utilisateur déconnecté");
    Redirect::to("/login")
}

/// API d'authentification pour les requêtes AJAX
pub async fn auth_status(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
    session: Session,
) -> impl IntoResponse {
    match get_current_user(&app_state.auth_service, &session).await {
        Ok(Some(user)) => {
            let response = AuthStatusResponse {
                authenticated: true,
                username: Some(user.username),
                must_change_password: user.must_change_password,
                is_admin: user.is_admin,
            };
            axum::Json(response).into_response()
        }
        Ok(None) => {
            let response = AuthStatusResponse {
                authenticated: false,
                username: None,
                must_change_password: false,
                is_admin: false,
            };
            axum::Json(response).into_response()
        }
        Err(e) => {
            error!("Erreur lors de la vérification du statut d'authentification: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// Fonctions utilitaires

fn render_login_error(handlebars: &Handlebars, error_message: &str) -> axum::response::Response {
    let context = json!({
        "error": error_message,
        "is_first_login": false,
        "default_password": ""
    });

    match handlebars.render("login", &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Erreur de rendu du template d'erreur de login: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn render_change_password_error(handlebars: &Handlebars, error_message: &str) -> axum::response::Response {
    let context = json!({
        "error": error_message
    });

    match handlebars.render("change-password", &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Erreur de rendu du template d'erreur de changement de mot de passe: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Connecte un utilisateur en session
async fn login_user(session: &Session, user: &User) -> Result<(), tower_sessions::session::Error> {
    session.insert("user_id", user.id.to_string()).await?;
    session.insert("username", &user.username).await?;
    session.insert("is_admin", user.is_admin).await?;
    session.insert("must_change_password", user.must_change_password).await?;
    Ok(())
}

/// Récupère l'utilisateur actuel depuis la session
pub async fn get_current_user(
    auth_service: &AuthService,
    session: &Session,
) -> Result<Option<User>, anyhow::Error> {
    if let Some(user_id_str) = session.get::<String>("user_id").await? {
        if let Ok(user_id) = Uuid::parse_str(&user_id_str) {
            return auth_service.get_user_by_id(user_id).await;
        }
    }
    Ok(None)
}

/// Endpoint pour l'initialisation du premier utilisateur (développement uniquement)
#[cfg(debug_assertions)]
pub async fn init_default_user(
    State(app_state): State<std::sync::Arc<crate::AppState>>,
) -> impl IntoResponse {
    match app_state.auth_service.initialize_default_user().await {
        Ok(password) => {
            if password != "Utilisateur existant" {
                info!("Utilisateur par défaut initialisé avec mot de passe: {}", password);
                axum::Json(json!({
                    "success": true,
                    "message": "Utilisateur créé",
                    "password": password
                })).into_response()
            } else {
                axum::Json(json!({
                    "success": true,
                    "message": "Utilisateur déjà existant"
                })).into_response()
            }
        }
        Err(e) => {
            error!("Erreur lors de l'initialisation de l'utilisateur par défaut: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}