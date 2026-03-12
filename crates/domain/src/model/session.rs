use crate::model::user::UserId;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct Session {
    pub id: SessionId,
    pub uid: UserId,
    pub jti: Uuid,
    pub token_hash: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

pub struct NewSession {
    pub uid: UserId,
    pub jti: Uuid,
    pub token_hash: String,
    pub expires_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}
