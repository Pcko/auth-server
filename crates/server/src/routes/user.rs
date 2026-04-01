use crate::dto::user_dto::UserResponseDTO;
use crate::middleware::admin_extractor::AdminExtractor;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use domain::repositories::user_repository::UserRepository;
use persistence::repositories::user_repository::DieselUserRepository;
use uuid::Uuid;
use crate::errors::api_error::ApiError;

async fn get_users(
    State(state): State<AppState>,
    _admin: AdminExtractor,
) -> Result<Json<Vec<UserResponseDTO>>, StatusCode> {
    let repo = DieselUserRepository::new(state.pool.clone());

    let users = repo
        .find_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = users
        .into_iter()
        .map(UserResponseDTO::from)
        .collect::<Vec<UserResponseDTO>>();

    Ok(Json(response))
}

async fn get_user(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    _admin: AdminExtractor,
) -> Result<impl IntoResponse, StatusCode> {
    let user = state
        .user_service
        .get_user(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dto = UserResponseDTO::from(user);
    Ok(Json(dto))
}

async fn elevate(
    State(state): State<AppState>,
    _admin: AdminExtractor,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state
        .user_service
        .get_user(id)
        .await
        .map_err(ApiError::from)?;

    state.admin_service
        .elevate_user(user)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::ACCEPTED)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/{id}", get(get_user))
        .route("/elevate/{id}", post(elevate))
}
