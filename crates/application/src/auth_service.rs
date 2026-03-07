use std::ptr::{null, null_mut};
use argon2::password_hash::{generate_salt, phc::PasswordHash};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use domain::model::user::{NewUser, User};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use persistence::models::user_row::NewUserRow;
use std::sync::Arc;
use thiserror::Error;
// DI per Domain UserRepository so the service doesn't know about diesel
#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        if email.trim().is_empty() {
            return Err(AuthError::Validation("email is empty".into()));
        }

        if username.trim().is_empty() {
            return Err(AuthError::Validation("username is empty".into()));
        }

        if password.len() < 8 {
            return Err(AuthError::Validation(
                "password must be at least 8 characters".into(),
            ));
        }

        // Hash the password
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes())
            .map_err(AuthError::Hash)?
            .to_string();

        // Create new User
        let new_user = NewUser {
            name: username,
            email: email,
            password_hash: password_hash,
        };

        let user = self.repo.save(&new_user).await.map_err(AuthError::Repo)?;
        Ok(user)
    }

    pub async fn login(&self, email: String, password: String) -> Result<User, AuthError> {
        if email.trim().is_empty() {
            return Err(AuthError::Validation("email is empty".into()));
        }
        if password.len() < 8 {
            return Err(AuthError::Validation(
                "password must be at least 8 characters".into(),
            ));
        }
        // find User if error during search -> internal else if not a user then user with email doesn't exist
        let user = self.repo
            .find_by_email(&email)
            .await
            .map_err(AuthError::Repo)?
            .ok_or(AuthError::InvalidCredentials("Email invalid".to_string()))?;

        // Hash the sent pw
        let argon2 = Argon2::default();
        let hashed_password = argon2
            .hash_password(password.as_bytes())
            .map_err(AuthError::Hash)?;
        
        // Hash comparison 
        argon2
            .verify_password(password.as_bytes(), &hashed_password)
            .map_err(AuthError::Hash)?;

        Ok(user)
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("Invalid Credentials : {0}")]
    InvalidCredentials(String),
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("repository error: {0}")]
    Repo(UserRepositoryError),
    #[error("hash error: {0}")]
    Hash(argon2::password_hash::Error),
}
