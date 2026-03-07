use crate::dto::user_dto::UserDTO;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use domain::repositories::user_repository::UserRepository;
use persistence::repositories::postgres_user_repository::DieselUserRepository;

async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserDTO>>, StatusCode> {
    let repo = DieselUserRepository::new(state.pool.clone());

    let users = repo
        .find_all()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = users
        .into_iter()
        .map(|user| UserDTO::from(user))
        .collect::<Vec<UserDTO>>();

    Ok(Json(response))
}

async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/{id}", get(get_user))
}
