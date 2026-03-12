use crate::utils::token_handler::TokenHandler;
use domain::model::Claims::Claims;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use secrecy::{SecretBox, SecretString};
use thiserror::Error;
use time::Duration;
use uuid::Uuid;

pub struct TokenService {
    header: Header,
}

impl TokenService {
    pub fn new() -> Self {
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        Self { header }
    }

    pub fn generate_access_token(
        &self,
        iss: String,
        sub: Uuid,
        aud: String,
        expires_in: Duration,
        secret: &[u8],
    ) -> Result<(String, Claims), TokenError> {
        let claims = Claims::new(iss, sub, aud, expires_in);
        let token = encode(&self.header, &claims, &EncodingKey::from_secret(secret))
            .map_err(|_| TokenError::Unexpected("Access Token couldn't be generated"))?;

        Ok((token, claims))
    }

    pub fn generate_refresh_token(&self) -> SecretBox<str> {
        let token = TokenHandler::generate_session_token();
        let refresh_token = SecretString::new(token.into_boxed_str());
        refresh_token
    }
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("token error: {0}")]
    Unexpected(&'static str),
}
