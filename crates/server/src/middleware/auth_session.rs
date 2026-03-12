use crate::errors::api_error::ApiError;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use tower_cookies::Cookies;
use uuid::Uuid;

pub struct AuthSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
}

pub const SESSION_COOKIE: &str = "session";

impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let unauthorized = || ApiError::Unauthorized("unauthorized".into());

        // Get the cookies from the request
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| unauthorized())?;

        // get session token out of session cookie
        let token = cookies
            .get(SESSION_COOKIE)
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| unauthorized())?;

        let session = app_state
            .auth_service
            .authenticate_session(&token, app_state.config.secret_key.as_ref())
            .await
            .map_err(|_| unauthorized())?;

        // Return data for routes
        Ok(AuthSession {
            user_id: session.uid.as_uuid(),
            session_id: session.id.as_uuid(),
        })
    }
}
