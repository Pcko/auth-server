use crate::errors::api_error::ApiError;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use tower_cookies::Cookies;

pub struct AdminExtractor {
    pub is_admin: bool,
}

pub const ACCESS_COOKIE_KEY: &str = "access";

impl<S> FromRequestParts<S> for AdminExtractor
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let unauthorized = || ApiError::Unauthorized("Unauthorized".to_string());

        // get the cookies from the request
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| unauthorized())?;

        // get the access_token
        let access_token = cookies
            .get(ACCESS_COOKIE_KEY)
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| unauthorized())?;

        let result = app_state
            .auth_service
            .is_admin(access_token.as_str(), &app_state.config.access_secret)
            .await;

        Ok(AdminExtractor { is_admin: result })
    }
}
