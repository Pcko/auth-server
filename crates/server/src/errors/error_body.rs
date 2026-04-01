use crate::errors::api_error::ApiError;
use application::services::auth_service::AuthError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/***
    ErrorBody serves to translate Errors directly to HTTP response codes
*/
#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(ErrorBody { message: msg })).into_response()
            }
            ApiError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody { message: msg }),
            )
                .into_response(),
            ApiError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorBody { message: msg })).into_response()
            }
            ApiError::Conflict(msg) => {
                (StatusCode::CONFLICT, Json(ErrorBody { message: msg })).into_response()
            }
            ApiError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, Json(ErrorBody { message: msg })).into_response()
            }
            ApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
