use uuid::Uuid;
use time::OffsetDateTime;

pub struct User {
    pub uid: UserId,
    pub uname: String,
    pub umail: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(value : Uuid) -> Self {
        Self(value)
    }
    
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}