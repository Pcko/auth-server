use crate::state::AppState;

use crate::dto::auth_dto::LoginDTO;
use crate::dto::register_dto::RegisterDTO;
use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use application::utils::token_handler::TokenHandler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use diesel_async::RunQueryDsl;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tower_cookies::cookie::time::OffsetDateTime;
use application::services::auth_service::LoginResult;

async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDTO>,
) -> impl IntoResponse {
    state
        .auth_service
        .register(dto.email, dto.username, dto.password)
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::CREATED)
}

async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(dto): Json<LoginDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state
        .auth_service
        /// TODO IMPLEMENT URL EXTRACTOR
        .login(dto.email, dto.password, "url".to_string(), state.config.session_secret.as_ref(), state.config.refresh_secret.as_ref())
        .await
        .map_err(ApiError::from)?;

    // Set cookie with session token on client
    let mut session = Cookie::new("session", result.session_token);
    configure_cookie(&state, result.session_expires_at, &mut session);
    let mut refresh = Cookie::new("refresh", result.refresh_token.into());
    configure_cookie(&state, result.refresh_expires_at, &mut refresh);
    
    cookies.add(session);
    cookies.add(refresh);
    // Convert to UserDTO so I dont expose internal data
    Ok((StatusCode::OK, Json(UserResponseDTO::from(result.user))))
}

// 
/**
    Configures Cookies to be secure (for DRY)
*/
fn configure_cookie(state: &AppState, expires_at: OffsetDateTime, cookie: &mut Cookie) {
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(state.config.is_dev);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(expires_at);
}

async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    // see if cookie even has the right value
    if let Some(cookie) = cookies.get("session") {
        let token_hash = TokenHandler::hash_token(
            &cookie.value().to_string(),
            state.config.session_secret.as_ref(),
        );
        state
            .auth_service
            .logout(token_hash.to_string())
            .await
            .map_err(ApiError::from)?;
    }
    // Remove token from client cookies
    let mut removal = Cookie::new("session", "");
    removal.set_path("/");
    removal.set_http_only(true);
    removal.set_same_site(SameSite::Lax);
    removal.set_secure(!state.config.is_dev);
    removal.make_removal();
    cookies.add(removal);

    Ok(StatusCode::NO_CONTENT)
}

async fn authenticate(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(cookie) = cookies.get("session") {
        state
            .auth_service
            .authenticate_session(cookie.value(), state.config.session_secret.as_ref())
            .await
            .map_err(ApiError::from)?;
    }

    Ok(StatusCode::OK)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/authenticate", post(authenticate))
}
