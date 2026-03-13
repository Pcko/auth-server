use argon2::{PasswordHash, PasswordHasher};
use base64::prelude::*;
use hmac::digest::Digest;
use hmac::{Hmac, Mac};
use rand::Rng;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

#[derive(Debug)]
pub struct TokenHandler {}

type HmacSha256 = Hmac<Sha256>;

impl TokenHandler {
    pub fn generate_token() -> String {
        let mut bytes: [u8; 32] = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        BASE64_URL_SAFE_NO_PAD.encode(bytes)
    }

    pub fn hash_token(token: &str, secret: &[u8]) -> String {
        let mut mac = HmacSha256::new_from_slice(secret).expect("valid key");
        mac.update(token.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
}
