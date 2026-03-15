use application::services::auth_service::AuthError;

/***
   ApiError serves to translate DB Errors from the persistence layer to server layer errors
*/
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    InternalServerError(String),
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
            AuthError::Token(err) => ApiError::Unauthorized(format!("Token error: {err}")),
        }
    }
}
