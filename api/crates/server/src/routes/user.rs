use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use crate::errors::error_body::{DocumentedApiError, ErrorBody, documented};
use crate::middleware::admin_extractor::AdminExtractor;
use crate::state::AppState;
use aide::NoApi;
use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use domain::repositories::user_repository::UserRepository;
use persistence::repositories::user_repository::DieselUserRepository;
use uuid::Uuid;

type JsonResult<T> = Result<Json<T>, DocumentedApiError>;
type StatusResult = Result<StatusCode, DocumentedApiError>;

async fn get_users(
    State(state): State<AppState>,
    NoApi(_admin): NoApi<AdminExtractor>,
) -> JsonResult<Vec<UserResponseDTO>> {
    let repo = DieselUserRepository::new(state.pool.clone());

    let users = repo
        .find_all()
        .await
        .map_err(|_| ApiError::InternalServerError("Internal server error".to_string()))
        .map_err(documented)?;

    let response = users
        .into_iter()
        .map(UserResponseDTO::from)
        .collect::<Vec<_>>();

    Ok(Json(response))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    NoApi(_admin): NoApi<AdminExtractor>,
) -> JsonResult<UserResponseDTO> {
    let user = state
        .user_service
        .get_user(id)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(Json(UserResponseDTO::from(user)))
}

async fn elevate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    NoApi(_admin): NoApi<AdminExtractor>,
) -> StatusResult {
    let user = state
        .user_service
        .get_user(id)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    state
        .admin_service
        .elevate_user(user)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::ACCEPTED)
}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_users, |op| {
                op.description("List all users. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<200, Json<Vec<UserResponseDTO>>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/{id}",
            get_with(get_user, |op| {
                op.description("Fetch a single user by id. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<200, Json<UserResponseDTO>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<404, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/elevate/{id}",
            post_with(elevate, |op| {
                op.description("Grant admin rights to a user. (Admin)")
                    .security_requirement("accessCookie")
                    .response::<202, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<403, Json<ErrorBody>>()
                    .response::<404, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
}
