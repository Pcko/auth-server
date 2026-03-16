use crate::utils::token_handler::TokenHandler;
use domain::model::Claims::Claims;
use domain::model::session::SessionId;
use domain::repositories::session_repository::SessionRepository;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use persistence::repositories::session_repository::DieselSessionRepository;
use secrecy::{SecretBox, SecretString};
use std::sync::Arc;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub struct TokenService {
    header: Header,
    validation: Validation,
    session_repository: Arc<DieselSessionRepository>,
}

impl TokenService {
    pub fn new(session_repository: Arc<DieselSessionRepository>) -> Self {
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
        iss - Issuer of token (f.e. "auth_server)
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
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("token error: {0}")]
    Unexpected(&'static str),
    #[error("token error: {0}")]
    InvalidToken(String),
}
