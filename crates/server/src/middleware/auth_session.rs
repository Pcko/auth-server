use crate::errors::api_error::ApiError;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use tower_cookies::Cookies;
use uuid::Uuid;

pub struct AuthSession {
    pub user_id: Uuid,
}

pub const ACCESS_COOKIE_KEY: &str = "access";

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

        // get access token out of access cookie
        let token = cookies
            .get(ACCESS_COOKIE_KEY)
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| unauthorized())?;

        let result = app_state
            .auth_service
            .verify_token(&*token, app_state.config.access_secret.as_slice())?;

        // Return data for routes
        Ok(AuthSession {
            user_id: result.uid,
        })
    }
}
