use application::auth_service::AuthError;

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
            AuthError::InvalidCredentials(msg) => ApiError::BadRequest(msg),
            AuthError::EmailAlreadyExists => {
                ApiError::Conflict(String::from("user with that email already exists"))
            }
            AuthError::Validation(msg) => ApiError::Unauthorized("Invalid email or password".to_string()),
            AuthError::Repo(_) => ApiError::InternalServerError("Internal server error".to_string()),
            AuthError::Hash(_) => ApiError::InternalServerError("Internal server error".to_string()),
        }
    }
}