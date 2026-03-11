use std::io::Read;
use crate::utils::token_generator::TokenHandler;
use argon2::password_hash::phc::PasswordHash;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use domain::model::session::{NewSession, Session, SessionId};
use domain::model::user::{NewUser, User};
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::sync::Arc;
use jsonwebtoken::{Algorithm, Header};
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tracing::{error, info, instrument};

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

pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
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

    #[instrument(name = "auth.register", skip(self, password), fields(email = %email, username = %email
    ))]
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        if email.trim().is_empty() {
            let msg: &'static str = "Email is empty";
            error!(msg);
            return Err(AuthError::Validation(msg.into()));
        }

        if username.trim().is_empty() {
            let msg: &'static str = "Username is empty";
            error!(msg);
            return Err(AuthError::Validation(msg.into()));
        }

        if password.len() < 8 {
            let msg: &'static str = "Password is too short";
            error!(msg);
            return Err(AuthError::Validation(msg.into()));
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

        info!(
            "User {0}: {1} created",
            user.uid.as_uuid().to_string(),
            user.uname
        );
        Ok(user)
    }

    #[instrument(name = "auth.login", skip(self, password, secret), fields(email = %email))]
    pub async fn login(
        &self,
        email: String,
        password: String,
        secret: &[u8],
    ) -> Result<LoginResult, AuthError> {
        if email.trim().is_empty() {
            let msg: &'static str = "Email is empty";
            error!(msg);
            return Err(AuthError::Validation(msg.into()));
        }
        if password.len() < 8 {
            let msg: &'static str = "Password is too short";
            error!(msg);
            return Err(AuthError::Validation(msg.into()));
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
            .map_err(|_| {
                error!("User:{} Invalid password", user.uid.as_uuid().to_string());
                AuthError::InvalidCredentials("invalid email or password".to_string())
            })?;

        // generate session and refresh token
        let mut header = Header::new(Algorithm::RS256);
        header.typ = Some("JWT".to_string());

        let now = OffsetDateTime::now_utc();
        let exp = (now + Duration::hours(1));
        let secret_as_string = String::from_utf8(secret.to_vec()).map_err(|_| AuthError::Unexpected)?;
        let claims = Claims {
            iss: secret_as_string,
            sub:
            exp: exp.unix_timestamp(),

        }


        // create session and with refresh token
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

        info!(
            user_id = %user.uid.as_uuid().to_string(),
            session_id = %new_session.id.as_uuid().to_string(),
            "login succeeded"
        );

        Ok(LoginResult {
            session_token: token,
            expires_at: expire_date,
            user: user,
        })
    }

    #[instrument(name = "auth.logout", skip(self, token_hash))]
    pub async fn logout(&self, token_hash: String) -> Result<(), AuthError> {
        self.session_repo
            .delete_by_token_hash(token_hash)
            .await
            .map_err(AuthError::SessionRepo)?;

        Ok(())
    }

    #[instrument(name = "auth.authenticate", skip(self, token, secret))]
    pub async fn authenticate_session(
        &self,
        token: &str,
        secret: &[u8],
    ) -> Result<Session, AuthError> {
        let hashed_token = TokenHandler::hash_token(token, secret);

        let session = self
            .session_repo
            .find_by_token_hash(hashed_token.to_string())
            .await
            .map_err(AuthError::SessionRepo)?
            .ok_or_else(|| AuthError::Authentication)?;

        if OffsetDateTime::now_utc() > session.expires_at {
            Err(AuthError::Authentication)?;
        }

        info!(
            "User {0}: {1}",
            session.user_id.as_uuid(),
            session.id.as_uuid().to_string()
        );
        Ok(session)
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
