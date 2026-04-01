use crate::utils::token_handler::TokenHandler;
use domain::model::claims::Claims;
use domain::model::session::{Session, SessionId};
use domain::repositories::session_repository::SessionRepository;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use secrecy::{ExposeSecret, SecretBox, SecretString};
use std::ops::Add;
use std::sync::Arc;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub struct TokenService {
    header: Header,
    validation: Validation,
    session_repository: Arc<dyn SessionRepository>,
}

pub struct RotatedRefreshToken {
    pub session: Session,
    pub refresh_token: SecretString,
}

impl TokenService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        let mut header = Header::new(Algorithm::HS256);
        let validation = Validation::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        Self {
            header,
            validation,
            session_repository,
        }
    }

    /***
    Access tokens are JWT tokens and therefore carry information
        iss - Issuer of token (f.e. auth_server)
        sub - Subject of token (UID)
        aud - Audience of token (f.e. my-app.com)
        expires_in - Token expire timestamp
        secret - Server side secret to salt token
     */
    pub fn generate_access_token(
        &self,
        iss: String,
        sub: Uuid,
        aud: String,
        sid: Uuid,
        expires_in: Duration,
        secret: &[u8],
    ) -> Result<(String, Claims), TokenError> {
        let claims = Claims::new(iss, sub, sid, aud, expires_in);
        let token = encode(&self.header, &claims, &EncodingKey::from_secret(secret))
            .map_err(|_| TokenError::Unexpected("Access Token couldn't be generated"))?;

        Ok((token, claims))
    }

    /***
       Refresh token are opaque strings
    */
    pub fn generate_refresh_token(&self) -> SecretBox<str> {
        let token = TokenHandler::generate_token();
        let refresh_token = SecretString::new(token.into_boxed_str());
        refresh_token
    }

    pub async fn verify_access_token(
        &self,
        token: &str,
        secret: &[u8],
    ) -> Result<Claims, TokenError> {
        let result =
            decode::<Claims>(token, &DecodingKey::from_secret(secret), &self.validation)
                .map_err(|_| TokenError::InvalidToken("Access Token is invalid".to_string()))?;

        let claims = result.claims;

        let session = self
            .session_repository
            .find_by_id(SessionId::new(claims.sid))
            .await
            .map_err(|_| TokenError::Unexpected("Session Repository Error"))?;

        let Some(session) = session else {
            return Err(TokenError::InvalidToken("Access Token Invalid".to_string()));
        };

        if session.revoked_at.is_some() || session.expires_at <= OffsetDateTime::now_utc() {
            return Err(TokenError::InvalidToken("Access Token Invalid".to_string()));
        }

        Ok(claims)
    }

    pub async fn rotate_refresh_token(
        &self,
        session: Session,
        secret: &[u8],
        refresh_token_duration: Duration,
    ) -> Result<RotatedRefreshToken, TokenError> {
        if session.revoked_at.is_some() || session.expires_at <= OffsetDateTime::now_utc() {
            return Err(TokenError::InvalidToken("Refresh Token invalid".into()));
        }

        let mut new_session = session;
        let refresh_token = self.generate_refresh_token();
        let refresh_token_hash = TokenHandler::hash_token(refresh_token.expose_secret(), secret);
        new_session.token_hash = refresh_token_hash;

        // This is for Session sliding (each refresh)
        let candidate = OffsetDateTime::now_utc().add(refresh_token_duration);
        // The token can live for 90 days at max
        let absolute_cap = new_session.created_at + Duration::days(90);
        let new_expires_at = candidate.min(absolute_cap);

        new_session.expires_at = new_expires_at;

        let updated_session = self
            .session_repository
            .update_refresh_token_data(new_session)
            .await
            .map_err(|_| TokenError::Unexpected("Token refresh failed"))?;

        Ok(RotatedRefreshToken {
            session: updated_session,
            refresh_token,
        })
    }
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("token error: {0}")]
    Unexpected(&'static str),
    #[error("token error: {0}")]
    InvalidToken(String),
}
