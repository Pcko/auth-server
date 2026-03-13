use crate::utils::token_handler::TokenHandler;
use domain::model::Claims::Claims;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use secrecy::{SecretBox, SecretString};
use thiserror::Error;
use time::Duration;
use uuid::Uuid;

pub struct TokenService {
    header: Header,
    validation: Validation,
}

impl TokenService {
    pub fn new() -> Self {
        let mut header = Header::new(Algorithm::HS256);
        let validation = Validation::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());

        Self { header, validation }
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

    pub fn verify_access_token(&self, token: &str, secret: &[u8]) -> Result<Claims, TokenError> {
        let result = decode::<Claims>(token, &DecodingKey::from_secret(secret), &self.validation);

        if (!result.is_ok()) {
            Err(TokenError::InvalidToken(
                "Access Token is invalid".to_string(),
            ))?;
        }

        let claims = result.unwrap().claims;
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
