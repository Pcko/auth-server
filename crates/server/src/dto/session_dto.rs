use domain::model::session::Session;
use schemars::JsonSchema;
use serde::Serialize;
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Serialize, JsonSchema)]
pub struct SessionDTO {
    pub id: String,
    pub uid: String,
    pub created_at: String,
    pub expires_at: String,
    pub last_seen_at: String,
    pub revoked_at: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

impl From<Session> for SessionDTO {
    fn from(session: Session) -> Self {
        Self {
            id: session.id.to_string(),
            uid: session.uid.to_string(),
            created_at: session
                .created_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| session.created_at.to_string()),
            expires_at: session
                .expires_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| session.expires_at.to_string()),
            last_seen_at: session
                .last_seen_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| session.last_seen_at.to_string()),
            revoked_at: session.revoked_at.map(|value| {
                value
                    .format(&Rfc3339)
                    .unwrap_or_else(|_| value.to_string())
            }),
            user_agent: session.user_agent,
            ip_address: session.ip_address,
        }
    }
}
