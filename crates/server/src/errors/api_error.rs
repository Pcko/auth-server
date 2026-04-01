use application::services::auth_service::AuthError;
use application::services::session_service::SessionError;

/***
   ApiError serves to translate DB Errors from the persistence layer to server layer errors
*/
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    InternalServerError(String),
    NotFound,
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::Authentication => ApiError::Unauthorized("Unauthorized".to_string()),
            AuthError::EmailAlreadyExists => {
                ApiError::Conflict("user with that email already exists".to_string())
            }
            AuthError::InvalidCredentials(_) | AuthError::Validation(_) => {
                ApiError::Unauthorized("Invalid email or password".to_string())
            }
            AuthError::UserRepo(_)
            | AuthError::SessionRepo(_)
            | AuthError::Hash(_)
            | AuthError::Unexpected(_)
            | AuthError::HashParse(_) => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
            AuthError::Token(_) => ApiError::Unauthorized("Token error".to_string()),
            AuthError::InvalidSession => ApiError::Unauthorized("Session error".to_string()),
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::NotFound => ApiError::NotFound,
            SessionError::Forbidden => ApiError::Forbidden("Forbidden".to_string()),
            SessionError::SessionRepo(_) | SessionError::Unexpected => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
        }
    }
}
