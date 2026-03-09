use crate::state::AppState;

use crate::dto::auth_dto::{LoginDTO, LogoutDTO};
use crate::dto::register_dto::RegisterDTO;
use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use application::utils::token_generator::TokenHandler;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::post;
use axum::{Json, Router, response};
use tower_cookies::cookie::{CookieJar, SameSite};
use tower_cookies::{Cookie, Cookies};

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
    mut cookies: Cookies,
    Json(dto): Json<LoginDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state
        .auth_service
        .login(dto.email, dto.password)
        .await
        .map_err(ApiError::from)?;
    
    // Set cookie with session token on client
    let mut cookie = Cookie::new("session", result.session_token);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(result.expires_at);

    cookies.add(cookie);

    // Convert to UserDTO so I dont expose internal data
    Ok((StatusCode::OK, Json(UserResponseDTO::from(result.user))))
}

async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    // see if cookie even has the right value
    if let Some(cookie) = cookies.get("session") {
        let token_hash = TokenHandler::hash_token(&cookie.value().to_string());
        state
            .auth_service
            .logout(token_hash.to_string())
            .await
            .map_err(ApiError::from)?;
    }

    // Remove token from client cookies
    let mut removal = Cookie::new("session", "");
    removal.set_path("/");
    removal.make_removal();

    cookies.add(removal);

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}
