use crate::services::token_service::{TokenError, TokenService};
use crate::utils::token_handler::TokenHandler;
use argon2::password_hash::phc::PasswordHash;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use domain::model::request_info::RequestInfo;
use domain::model::session::{NewSession, SessionId};
use domain::model::user::{NewUser, User};
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tracing::{error, info, instrument, warn};
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

pub struct RefreshResult {
    pub access_token: String,
    pub access_expires_at: OffsetDateTime,
    pub refresh_token: SecretString,
    pub refresh_expires_at: OffsetDateTime,
}

const ACCESS_TOKEN_DURATION: Duration = Duration::hours(1);
const REFRESH_TOKEN_DURATION: Duration = Duration::days(30);

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

    #[instrument(name = "auth.register", skip(self, password), fields(email = %email, username = %username))]
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        Self::validate_credentials(&email, &password)?;

        if username.trim().is_empty() {
            let msg: &'static str = "Username is empty";
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
            role: None,
        };

        let user = self
            .user_repo
            .save(&new_row)
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
        request_info: RequestInfo,
        access_secret: &[u8],
        refresh_secret: &[u8],
        audience: &str,
        iss: &str,
    ) -> Result<LoginResult, AuthError> {
        info!("login handler entered");
        Self::validate_credentials(&email, &password)?;

        // find User if error during search -> internal else if not a user then user with email doesn't exist
        let user = self
            .user_repo
            .find_by_email(&email)
            .await
            .map_err(AuthError::UserRepo)?
            .ok_or(AuthError::InvalidCredentials(
                "invalid email or password".to_string(),
            ))?;

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
        let refresh_expires_at = OffsetDateTime::now_utc() + REFRESH_TOKEN_DURATION;

        // create session and with refresh token
        let session = NewSession {
            uid: user.uid,
            expires_at: refresh_expires_at,
            token_hash: refresh_token_hash,
            user_agent: request_info.user_agent,
            ip_address: request_info.ip,
            revoked_at: None,
        };

        let new_session = self
            .session_repo
            .insert(session)
            .await
            .map_err(AuthError::SessionRepo)?;
        info!("session created");

        // access token
        let (access_token, claims) = self
            .token_service
            .generate_access_token(
                iss.to_string(),
                user.uid.as_uuid(),
                audience.to_string(),
                new_session.id.as_uuid(),
                ACCESS_TOKEN_DURATION,
                access_secret,
            )
            .map_err(AuthError::Token)?;

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

    fn validate_credentials(email: &str, password: &str) -> Result<(), AuthError> {
        if email.trim().is_empty() {
            return Err(AuthError::Validation("Email is empty".into()));
        }

        if password.len() < 8 {
            return Err(AuthError::Validation("Password is too short".into()));
        }

        Ok(())
    }

    #[instrument(
        name = "auth.logout",
        skip(self, access_cookie, access_secret, refresh_token, refresh_secret)
    )]
    pub async fn logout(
        &self,
        access_cookie: Option<String>,
        access_secret: &[u8],
        refresh_token: Option<String>,
        refresh_secret: &[u8],
    ) -> Result<(), AuthError> {
        // extract sid from access token if exists
        let sid_opt = match access_cookie.as_deref() {
            Some(access_token) if !access_token.is_empty() => self
                .token_service
                .verify_access_token(access_token, access_secret)
                .await
                .ok()
                .map(|claims| SessionId::new(claims.sid)),
            _ => None,
        };

        // get session from refresh token if exists
        let session_opt = match refresh_token.as_deref() {
            Some(refresh_token) if !refresh_token.is_empty() => {
                let token_hash = TokenHandler::hash_token(refresh_token, refresh_secret);

                self.session_repo
                    .find_by_token_hash(token_hash)
                    .await
                    .map_err(AuthError::SessionRepo)?
            }
            _ => None,
        };

        // if a session rly exists delete token
        if let Some(session) = session_opt {
            // if sid of both tokens dont match up
            if let Some(sid) = sid_opt
                && sid != session.id
            {
                // TODO implement suspicious flagging or smth
                warn!("user {0}: logout token mismatch", session.uid)
            }

            match self.session_repo.delete_by_id(session.id).await {
                Ok(_) | Err(SessionRepositoryError::NotFound) => {}
                Err(err) => return Err(AuthError::SessionRepo(err)),
            }

            info!("session {} was deleted", session.id);
            return Ok(());
        }

        Ok(())
    }

    async fn sid_from_refresh_token(
        &self,
        refresh_token: &str,
        refresh_secret: &[u8],
    ) -> Result<SessionId, AuthError> {
        let refresh_token_hash = TokenHandler::hash_token(refresh_token, refresh_secret);

        let session = self
            .session_repo
            .find_by_token_hash(refresh_token_hash)
            .await
            .map_err(AuthError::SessionRepo)?
            .ok_or(AuthError::InvalidSession)?;

        Ok(session.id)
    }

    #[instrument(name = "auth.verify_token", skip(self, access_token, access_secret))]
    pub async fn verify_token(
        &self,
        access_token: &str,
        access_secret: &[u8],
        iss: &str,
    ) -> Result<VerifyResult, AuthError> {
        let claims = self
            .token_service
            .verify_access_token(access_token, access_secret)
            .await
            .map_err(AuthError::Token)?;

        if OffsetDateTime::now_utc() > claims.exp {
            return Err(AuthError::Authentication);
        }

        if claims.iss != iss {
            return Err(AuthError::Authentication);
        }

        info!("User {0}: {1}", claims.sub, claims.jti);

        Ok(VerifyResult {
            uid: claims.sub,
            sid: claims.sid,
        })
    }

    #[instrument(name = "auth.refresh_token", skip(self, refresh_token, refresh_secret))]
    pub async fn refresh_token(
        &self,
        aud: &str,
        refresh_token: &str,
        refresh_secret: &[u8],
        access_secret: &[u8],
        iss: &str,
    ) -> Result<RefreshResult, AuthError> {
        let hashed_token = TokenHandler::hash_token(refresh_token, refresh_secret);

        let session_option = self
            .session_repo
            .find_by_token_hash(hashed_token)
            .await
            .map_err(AuthError::SessionRepo)?;

        let Some(session) = session_option else {
            return Err(AuthError::Authentication);
        };

        if session.revoked_at.is_some() || session.expires_at <= OffsetDateTime::now_utc() {
            return Err(AuthError::Authentication);
        }

        let (access_token, claims) = self
            .token_service
            .generate_access_token(
                iss.to_string(),
                session.uid.as_uuid(),
                aud.to_string(),
                session.id.as_uuid(),
                ACCESS_TOKEN_DURATION,
                access_secret,
            )
            .map_err(AuthError::Token)?;

        let result = self
            .token_service
            .rotate_refresh_token(session, refresh_secret, REFRESH_TOKEN_DURATION)
            .await?;
        info!(
            "Token refreshed and RToken rotated: {0} {1}",
            result.session.id, result.session.uid
        );

        Ok(RefreshResult {
            access_token,
            access_expires_at: claims.exp,
            refresh_token: result.refresh_token,
            refresh_expires_at: result.session.expires_at,
        })
    }

    pub async fn is_admin(&self, access_token: &str, access_secret: &[u8]) -> bool {
        let claims = self
            .token_service
            .verify_access_token(access_token, access_secret)
            .await
            .map_err(AuthError::Token);

        if let Ok(claims) = claims {
            claims.is_admin
        } else {
            false
        }
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
    #[error("token error: {0}")]
    Token(#[from] TokenError),
    #[error("invalid session error")]
    InvalidSession,
}
