use crate::errors::api_error::ApiError;
use aide::{IntoApi, UseApi};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use schemars::JsonSchema;
use serde::Serialize;

/***
    ErrorBody serves to translate Errors directly to HTTP response codes
*/
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ErrorBody {
    pub message: String,
}

pub type DocumentedApiError = UseApi<ApiError, Json<ErrorBody>>;

impl ErrorBody {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub fn documented(error: ApiError) -> DocumentedApiError {
    error.into_api::<Json<ErrorBody>>()
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(ErrorBody::new(msg))).into_response()
            }
            ApiError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::new(msg)),
            )
                .into_response(),
            ApiError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorBody::new(msg))).into_response()
            }
            ApiError::Conflict(msg) => {
                (StatusCode::CONFLICT, Json(ErrorBody::new(msg))).into_response()
            }
            ApiError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, Json(ErrorBody::new(msg))).into_response()
            }
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, Json(ErrorBody::new("Not found"))).into_response()
            }
        }
    }
}
