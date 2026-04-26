use crate::dto::oauth_dto::{TokenRequest, TokenResponse};
use crate::errors::api_error::ApiError;
use crate::state::AppState;
use aide::axum::ApiRouter;
use axum::Json;
use axum::extract::State;

pub fn authorize(
    State(state): State<AppState>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    if req.grant_type != "authorization_code" {
        return Err(ApiError::BadRequest("unsupported grant_type".to_string()));
    }


}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("", |op| {
        op.description("Creates one-time code for Client sessions.")
            .response::<200, TokenResponse>()
    })
}
