use serde::Serialize;
use time::OffsetDateTime;

use domain::model::session::Session;

#[derive(Debug, Serialize)]
pub struct SessionDTO {
    pub id: String,
    pub uid: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

impl From<Session> for SessionDTO {
    fn from(session: Session) -> Self {
        Self {
            id: session.id.to_string(),
            uid: session.uid.to_string(),
            created_at: session.created_at,
            expires_at: session.expires_at,
            last_seen_at: session.last_seen_at,
            revoked_at: session.revoked_at,
            user_agent: session.user_agent,
            ip_address: session.ip_address,
        }
    }
}