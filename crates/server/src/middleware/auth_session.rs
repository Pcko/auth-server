use crate::errors::api_error::ApiError;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use tower_cookies::Cookies;

pub struct AuthSession {
    pub session_id: String,
    pub user_id: String,
}

impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Get the cookies from the request
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::Unauthorized("could not read cookies".to_string()))?;

        // get session token out of session cookie
        let token = cookies
            .get("session")
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| ApiError::Unauthorized("missing session cookie".to_string()))?;
    
        let session = app_state
            .auth_service
            .authenticate_session(&token, app_state.config.secret_key.as_ref())
            .await
            .map_err(|_| ApiError::Unauthorized("invalid session".to_string()))?;

        // Return data for routes
        Ok(AuthSession {
            user_id: session.user_id.as_uuid().to_string(),
            session_id: session.id.as_uuid().to_string(),
        })
    }
}