use crate::dto::session_dto::SessionDTO;
use crate::errors::api_error::ApiError;
use crate::middleware::admin_extractor::AdminExtractor;
use crate::middleware::user_extractor::UserExtractor;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::{Json, Router};
use http::StatusCode;
use uuid::Uuid;

pub async fn get_all_sessions(
    State(state): State<AppState>,
    _admin: AdminExtractor,
) -> Result<impl IntoResponse, ApiError> {
    let result = state
        .session_service
        .get_all()
        .await
        .map_err(ApiError::from)?;

    let dtos = result.into_iter().map(SessionDTO::from).collect::<Vec<_>>();
    Ok(Json(dtos))
}

pub async fn revoke_session(
    State(state): State<AppState>,
    _admin: AdminExtractor,
    Path(sid): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .session_service
        .delete(sid)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_my_sessions(
    State(state): State<AppState>,
    user: UserExtractor,
) -> Result<impl IntoResponse, ApiError> {
    let result = state
        .session_service
        .get_sessions_by_uid(user.uid)
        .await
        .map_err(ApiError::from)?;

    let dtos = result.into_iter().map(SessionDTO::from).collect::<Vec<_>>();
    Ok(Json(dtos))
}

pub async fn revoke_all_my_sessions(
    State(state): State<AppState>,
    user: UserExtractor,
) -> Result<impl IntoResponse, ApiError> {
    state
        .session_service
        .delete_by_uid(user.uid)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn revoke_my_session(
    State(state): State<AppState>,
    Path(sid): Path<Uuid>,
    user: UserExtractor,
) -> Result<impl IntoResponse, ApiError> {
    let session = state
        .session_service
        .get_by_session_id(sid)
        .await
        .map_err(ApiError::from)?;

    let Some(session) = session else {
        Err(ApiError::NotFound)?
    };

    if session.uid.as_uuid() != user.uid {
        Err(ApiError::Forbidden("Forbidden".to_string()))?
    }

    state
        .session_service
        .delete(sid)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_user_sessions(
    State(state): State<AppState>,
    Path(uid): Path<Uuid>,
    _admin: AdminExtractor,
) -> Result<impl IntoResponse, ApiError> {
    let sessions = state
        .session_service
        .get_sessions_by_uid(uid)
        .await
        .map_err(ApiError::from)?;

    let dtos = sessions
        .into_iter()
        .map(SessionDTO::from)
        .collect::<Vec<_>>();

    Ok(Json(dtos))
}

pub async fn revoke_all_user_sessions(
    State(state): State<AppState>,
    Path(uid): Path<Uuid>,
    _admin: AdminExtractor,
) -> Result<impl IntoResponse, ApiError> {
    state
        .session_service
        .delete_by_uid(uid)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

// default session routes
pub fn user_session_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_sessions))
        .route("/", delete(revoke_all_my_sessions))
        .route("/{sid}", delete(revoke_my_session))
}

// admin session routes
pub fn admin_session_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_sessions))
        .route("/{sid}", delete(revoke_session))
        .route("/user/{uid}", get(get_all_user_sessions))
        .route("/user/{uid}", delete(revoke_all_user_sessions))
}
