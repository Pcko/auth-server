use diesel::{Insertable, Queryable, Selectable};
use domain::model::session::{NewSession, Session, SessionId};
use domain::model::user::UserId;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionRow {
    pub id: Uuid,
    pub uid: Uuid,
    pub token_hash: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::sessions)]
pub struct NewSessionRow<'a> {
    pub uid: Uuid,
    pub token_hash: &'a str,
    pub expires_at: &'a OffsetDateTime,
    pub user_agent: Option<&'a String>,
    pub ip_address: Option<&'a String>,
}

impl From<SessionRow> for Session {
    fn from(row: SessionRow) -> Self {
        Session {
            id: SessionId::new(row.id),
            user_id: UserId::new(row.uid),
            token_hash: row.token_hash,
            created_at: row.created_at,
            last_seen_at: row.last_seen_at,
            expires_at: row.expires_at,
            revoked_at: row.revoked_at,
            user_agent: row.user_agent,
            ip_address: row.ip_address,
        }
    }
}

impl<'a> From<&'a NewSession> for NewSessionRow<'a> {
    fn from(session: &'a NewSession) -> NewSessionRow<'a> {
        Self {
            uid: session.uid.as_uuid(),
            token_hash: &session.token_hash,
            expires_at: &session.expires_at,
            user_agent: session.user_agent.as_ref(),
            ip_address: session.ip_address.as_ref(),
        }
    }
}
