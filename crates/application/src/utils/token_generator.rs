use crate::services::auth_service::AuthError;
use argon2::{PasswordHash, PasswordHasher};
use base64::prelude::*;
use rand::RngCore;

#[derive(Debug)]
pub struct TokenHandler {}

impl TokenHandler {
    pub fn generate_session_token() -> String {
        let mut bytes: [u8; 32] = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        BASE64_URL_SAFE_NO_PAD.encode(bytes)
    }

    pub fn hash_token(token: &String) -> PasswordHash {
        let argon2 = argon2::Argon2::default();
        argon2
            .hash_password(token.as_bytes())
            .map_err(AuthError::Hash)
            .unwrap()
    }
}
