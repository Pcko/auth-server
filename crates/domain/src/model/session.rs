use time::OffsetDateTime;
use uuid::Uuid;

pub struct Session {
    id: Uuid,
    uid: Uuid,
    token_hash: String,
    created_at: OffsetDateTime,
    expires_at: OffsetDateTime,
    last_seen_at: OffsetDateTime,
    revoke_at: OffsetDateTime,
    user_agent: Option<String>,
    ip_address: Option<String>,
}
