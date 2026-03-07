use crate::state::AppState;

use crate::dto::login_dto::LoginDTO;
use crate::dto::register_dto::RegisterDTO;
use crate::dto::user_dto::UserDTO;
use crate::errors::api_error::ApiError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

async fn register(
    State(state): State<AppState>,
    Json(user): Json<RegisterDTO>,
) -> impl IntoResponse {
    state
        .auth_service
        .register(user.username, user.password, user.email)
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::CREATED)
}

async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginDTO>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state
        .auth_service
        .login(data.email, data.password)
        .await
        .map_err(ApiError::from)?;
    // Convert to UserDTO so I dont expose internal data
    Ok(Json(UserDTO::from(user)))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
