use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub jti: Uuid,
    pub iss: String,
    pub sub: Uuid,
    #[serde(with = "time::serde::timestamp")]
    pub iat: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub exp: OffsetDateTime,
    pub aud: String,
    pub sid: Uuid,
    pub is_admin: bool,
}

impl Claims {
    pub fn new(iss: String, sub: Uuid, sid: Uuid, aud: String, expires_in: Duration) -> Self {
        let now = OffsetDateTime::now_utc();
        let exp = now + expires_in;

        Self {
            jti: Uuid::new_v4(),
            iss: iss,
            sub: sub,
            iat: now,
            exp: exp,
            aud: aud,
            sid: sid,
            // TODO implementiere admin system
            is_admin: false,
        }
    }
}
