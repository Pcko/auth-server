use crate::state::AppState;

use crate::dto::auth_dto::LoginDTO;
use crate::dto::register_dto::RegisterDTO;
use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use application::utils::token_handler::TokenHandler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use secrecy::{ExposeSecret, SecretString};
use tower_cookies::cookie::SameSite;
use tower_cookies::cookie::time::OffsetDateTime;
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
    cookies: Cookies,
    Json(dto): Json<LoginDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state
        .auth_service
        // TODO IMPLEMENT URL EXTRACTOR for aud
        .login(
            dto.email,
            dto.password,
            "url".to_string(),
            state.config.access_secret.as_ref(),
            state.config.refresh_secret.as_ref(),
        )
        .await
        .map_err(ApiError::from)?;

    // Set cookie with session token on client
    let mut session = Cookie::new("access", result.access_token);
    configure_cookie(&state, result.access_expires_at, &mut session);
    let mut refresh = Cookie::new("refresh", result.refresh_token.expose_secret().to_owned());
    configure_cookie(&state, result.refresh_expires_at, &mut refresh);

    cookies.add(session);
    cookies.add(refresh);
    // Convert to UserDTO so I dont expose internal data
    Ok((StatusCode::OK, Json(UserResponseDTO::from(result.user))))
}

async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    let revoke_result = if let Some(refresh) = cookies.get("refresh") {
        state
            .auth_service
            .logout(
                refresh.value().to_string(),
                state.config.refresh_secret.as_ref(),
            )
            .await
    } else {
        Ok(())
    };

    let mut access_token_removal = Cookie::new("access", "");
    remove_cookie(&state, &mut access_token_removal);
    cookies.remove(access_token_removal);

    let mut refresh_token_removal = Cookie::new("refresh", "");
    remove_cookie(&state, &mut refresh_token_removal);
    cookies.remove(refresh_token_removal);

    match revoke_result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::error!(error = %err, "failed to revoke session during logout");
            Ok(StatusCode::NO_CONTENT)
        }
    }
}

async fn authenticate(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(cookie) = cookies.get("access") {
        let result = state
            .auth_service
            .verify_token(cookie.value(), state.config.access_secret.as_ref())
            .map_err(ApiError::from)?;

        return Ok((StatusCode::OK, result.uid));
    }

    Ok(StatusCode::UNAUTHORIZED)
}

async fn refresh(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(cookie) = cookies.get("refresh") {
        let refresh_token = cookie.value();
        // TODO IMPLEMENT URL EXTRACTOR for aud
        let result = state
            .auth_service
            .refresh_token(
                "".to_string(),
                refresh_token,
                state.config.refresh_secret.as_ref(),
                state.config.access_secret.as_ref(),
            )
            .await
            .map_err(ApiError::from)?;

        let mut cookie = Cookie::new("access", result.access_token);
        configure_cookie(&state, result.access_expires_at, &mut cookie);
        cookies.add(cookie);

        return Ok(StatusCode::OK);
    }

    Ok(StatusCode::UNAUTHORIZED)
}

/**
Configures Cookies to be secure (for DRY)
*/
fn configure_cookie(state: &AppState, expires_at: OffsetDateTime, cookie: &mut Cookie) {
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(!state.config.is_dev);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(expires_at);
}

/**
Configures Cookies for deletion
*/
fn remove_cookie(state: &AppState, cookie_to_remove: &mut Cookie) {
    cookie_to_remove.set_path("/");
    cookie_to_remove.set_http_only(true);
    cookie_to_remove.set_same_site(SameSite::Lax);
    cookie_to_remove.set_secure(!state.config.is_dev);
    cookie_to_remove.make_removal();
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(authenticate))
        .route("/refresh", post(refresh))
}
