use crate::dto::session_dto::SessionDTO;
use crate::errors::api_error::ApiError;
use crate::errors::error_body::{DocumentedApiError, ErrorBody, documented};
use crate::middleware::user_extractor::UserExtractor;
use crate::state::AppState;
use aide::NoApi;
use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with};
use axum::Json;
use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

type JsonResult<T> = Result<Json<T>, DocumentedApiError>;
type StatusResult = Result<StatusCode, DocumentedApiError>;

pub async fn get_all_sessions(State(state): State<AppState>) -> JsonResult<Vec<SessionDTO>> {
    let result = state
        .session_service
        .get_all()
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let dtos = result.into_iter().map(SessionDTO::from).collect::<Vec<_>>();
    Ok(Json(dtos))
}

pub async fn revoke_session(State(state): State<AppState>, Path(sid): Path<Uuid>) -> StatusResult {
    state
        .session_service
        .delete(sid)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_my_sessions(
    State(state): State<AppState>,
    NoApi(UserExtractor { user }): NoApi<UserExtractor>,
) -> JsonResult<Vec<SessionDTO>> {
    let result = state
        .session_service
        .get_sessions_by_uid(user.uid.as_uuid())
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let dtos = result.into_iter().map(SessionDTO::from).collect::<Vec<_>>();
    Ok(Json(dtos))
}

pub async fn revoke_all_my_sessions(
    State(state): State<AppState>,
    NoApi(UserExtractor { user }): NoApi<UserExtractor>,
) -> StatusResult {
    state
        .session_service
        .delete_by_uid(user.uid.as_uuid())
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn revoke_my_session(
    State(state): State<AppState>,
    Path(sid): Path<Uuid>,
    NoApi(UserExtractor { user }): NoApi<UserExtractor>,
) -> StatusResult {
    let session = state
        .session_service
        .get_by_session_id(sid)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let Some(session) = session else {
        return Err(documented(ApiError::NotFound));
    };

    if session.uid.as_uuid() != user.uid.as_uuid() {
        return Err(documented(ApiError::Forbidden("Forbidden".to_string())));
    }

    state
        .session_service
        .delete(sid)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_user_sessions(
    State(state): State<AppState>,
    Path(uid): Path<Uuid>,
    NoApi(_admin): NoApi<UserExtractor>,
) -> JsonResult<Vec<SessionDTO>> {
    let sessions = state
        .session_service
        .get_sessions_by_uid(uid)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let dtos = sessions
        .into_iter()
        .map(SessionDTO::from)
        .collect::<Vec<_>>();
    Ok(Json(dtos))
}

pub async fn revoke_all_user_sessions(
    State(state): State<AppState>,
    Path(uid): Path<Uuid>,
    NoApi(_admin): NoApi<UserExtractor>,
) -> StatusResult {
    state
        .session_service
        .delete_by_uid(uid)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn user_session_router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_my_sessions, |op| {
                op.description("List the sessions of requesting user.")
                    .security_requirement("accessCookie")
                    .response::<200, Json<Vec<SessionDTO>>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/",
            delete_with(revoke_all_my_sessions, |op| {
                op.description("Revoke all sessions for the requesting user.")
                    .security_requirement("accessCookie")
                    .response::<204, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/{sid}",
            delete_with(revoke_my_session, |op| {
                op.description("Revoke a single session owned by requesting user.")
                    .security_requirement("accessCookie")
                    .response::<204, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<404, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
}

pub fn admin_session_router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_all_sessions, |op| {
                op.description("List all sessions. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<200, Json<Vec<SessionDTO>>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/{sid}",
            delete_with(revoke_session, |op| {
                op.description("Revoke a session by id. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<204, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/user/{uid}",
            get_with(get_all_user_sessions, |op| {
                op.description("List all sessions for a specific user. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<200, Json<Vec<SessionDTO>>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/user/{uid}",
            delete_with(revoke_all_user_sessions, |op| {
                op.description("Revoke all sessions for a specific user. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<204, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
}
