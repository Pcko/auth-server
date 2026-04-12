use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use crate::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use domain::model::user::User;
use tower_cookies::Cookies;

pub struct UserExtractor {
    pub user: User,
}

impl<S> FromRequestParts<S> for UserExtractor
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
            .get("accessToken")
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(unauthorized)?;

        let result = app_state
            .auth_service
            .verify_token(
                &token,
                app_state.config.access_secret.as_slice(),
                &app_state.config.issuer,
            )
            .await?;

        let user = app_state.user_service.get_user(result.uid).await?;

        // Return data for routes
        Ok(UserExtractor { user })
    }
}
