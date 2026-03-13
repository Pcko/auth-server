use crate::services::token_service::TokenService;
use crate::utils::token_handler::TokenHandler;
use argon2::password_hash::phc::PasswordHash;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use domain::model::session::NewSession;
use domain::model::user::{NewUser, User};
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tracing::{error, info, instrument};
use uuid::Uuid;

// DI per Domain UserRepository so the service doesn't know about diesel
#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn SessionRepository>,
    token_service: Arc<TokenService>,
}

pub struct LoginResult {
    pub user: User,
    pub access_token: String,
    pub access_expires_at: OffsetDateTime,
    pub refresh_token: SecretString,
    pub refresh_expires_at: OffsetDateTime,
}

pub struct VerifyResult {
    pub uid: Uuid,
    pub sid: Uuid,
}

const ISSUER_NAME: &'static str = "AUTH_SERVER";

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn SessionRepository>,
        token_service: Arc<TokenService>,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            token_service,
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

        info!("User {0}: {1} created", user.uid.to_string(), user.uname);
        Ok(user)
    }

    #[instrument(name = "auth.login", skip(self, password, access_secret, refresh_secret), fields(email = %email
    ))]
    pub async fn login(
        &self,
        email: String,
        password: String,
        aud: String,
        access_secret: &[u8],
        refresh_secret: &[u8],
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
        let parsed_pw_hash =
            PasswordHash::new(&user.password_hash).map_err(AuthError::HashParse)?;

        // comparison
        argon2
            .verify_password(password.as_bytes(), &parsed_pw_hash)
            .map_err(|_| {
                error!("User:{} Invalid password", user.uid.to_string());
                AuthError::InvalidCredentials("invalid email or password".to_string())
            })?;

        // refresh token
        let refresh_token = self.token_service.generate_refresh_token();
        let refresh_token_hash =
            TokenHandler::hash_token(refresh_token.expose_secret(), refresh_secret);
        let refresh_expires_at = OffsetDateTime::now_utc() + Duration::days(30);

        // create session and with refresh token
        let session = NewSession {
            uid: user.uid,
            expires_at: refresh_expires_at,
            token_hash: String::from(refresh_token_hash),
            user_agent: None,
            ip_address: None,
            revoked_at: None,
        };

        let new_session = self
            .session_repo
            .insert(session)
            .await
            .map_err(AuthError::SessionRepo)?;

        // access token
        let (access_token, claims) = self
            .token_service
            .generate_access_token(
                ISSUER_NAME.to_string(),
                user.uid.as_uuid(),
                aud,
                new_session.id.as_uuid(),
                Duration::hours(1),
                access_secret,
            )
            .map_err(|_| AuthError::InvalidCredentials("invalid token".to_string()))?;

        info!(
            user_id = %user.uid.as_uuid(),
            session_id = %new_session.id,
            "login succeeded"
        );

        Ok(LoginResult {
            access_token,
            access_expires_at: claims.exp,
            refresh_token,
            refresh_expires_at,
            user,
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

    #[instrument(name = "auth.verify_token", skip(self, access_token, secret))]
    pub fn verify_token(
        &self,
        access_token: &str,
        secret: &[u8],
    ) -> Result<VerifyResult, AuthError> {
        let result = self.token_service.verify_access_token(access_token, secret);

        if (result.is_err()) {
            return Err(AuthError::InvalidCredentials(
                "invalid access token".to_string(),
            ));
        }

        let claims = result.unwrap();

        if OffsetDateTime::now_utc() > claims.exp {
            Err(AuthError::Authentication)?;
        }

        if !claims.iss.eq(ISSUER_NAME) {
            Err(AuthError::Authentication)?;
        }

        info!("User {0}: {1}", claims.sub, claims.jti);
        Ok(VerifyResult {
            uid: claims.sub,
            sid: claims.sid,
        })
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
