use crate::utils::token_generator::TokenHandler;
use argon2::password_hash::phc::PasswordHash;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use domain::model::session::{NewSession, Session, SessionId};
use domain::model::user::{NewUser, User};
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::error::Error;
use std::sync::Arc;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

// DI per Domain UserRepository so the service doesn't know about diesel
#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn SessionRepository>,
}

pub struct LoginResult {
    pub user: User,
    pub session_token: String,
    pub expires_at: OffsetDateTime,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
        }
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
        let new_row = NewUser {
            name: username,
            email: email,
            password_hash: password_hash,
        };

        let user = self
            .user_repo
            .save(&new_row.into())
            .await
            .map_err(AuthError::UserRepo)?;

        Ok(user)
    }

    pub async fn login(&self, email: String, password: String) -> Result<LoginResult, AuthError> {
        if email.trim().is_empty() {
            return Err(AuthError::Validation("email is empty".into()));
        }
        if password.len() < 8 {
            return Err(AuthError::Validation(
                "password must be at least 8 characters".into(),
            ));
        }
        // find User if error during search -> internal else if not a user then user with email doesn't exist
        let user = self
            .user_repo
            .find_by_email(&email)
            .await
            .map_err(AuthError::UserRepo)?
            .ok_or(AuthError::InvalidCredentials("Email invalid".to_string()))?;

        // parse the password from db
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(AuthError::HashParse)?;
      
        // comparison
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials("invalid email or password".to_string()))?;

        // session params such as rnd token
        let token = TokenHandler::generate_session_token();
        let hashed_token = TokenHandler::hash_token(&token);
        let expire_date = OffsetDateTime::now_utc() + Duration::minutes(30);

        // create session
        let session = NewSession {
            uid: user.uid,
            expires_at: expire_date,
            token_hash: hashed_token.to_string(),
            user_agent: None,
            ip_address: None,
            revoked_at: None,
        };

        let new_session = self
            .session_repo
            .insert(session)
            .await
            .map_err(AuthError::SessionRepo)?;

        Ok(LoginResult {
            session_token: token,
            expires_at: expire_date,
            user: user,
        })
    }

    pub async fn logout(&self, session_id: String) -> Result<(), AuthError> {
        // Convert String to Uuid
        let sid = Uuid::try_parse(&session_id).map_err(|e| AuthError::Unexpected(e.to_string()))?;

        self.session_repo
            .delete_by_id(SessionId::new(sid))
            .await
            .map_err(AuthError::SessionRepo)?;

        Ok(())
    }

    pub async fn authenticate_session(&self, token: &String) -> Result<Session, AuthError> {
        let hashed_token = TokenHandler::hash_token(token);

        let session = self
            .session_repo
            .find_by_token_hash(hashed_token.to_string())
            .await
            .map_err(AuthError::SessionRepo)?
            .ok_or_else(|| AuthError::Authentication);

        Ok(session?)
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("invalid Credentials : {0}")]
    InvalidCredentials(String),
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("repository error: {0}")]
    UserRepo(#[from] UserRepositoryError),
    #[error("repository error: {0}")]
    SessionRepo(#[from] SessionRepositoryError),
    #[error("hash error: {0}")]
    Hash(#[from] argon2::password_hash::Error),
    #[error("hash parse error: {0}")]
    HashParse(#[from] argon2::password_hash::phc::Error),
    #[error("error with authentication ")]
    Authentication,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
