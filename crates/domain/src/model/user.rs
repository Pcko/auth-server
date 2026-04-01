use std::fmt;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::model::user_type::UserRole;

pub struct User {
    pub uid: UserId,
    pub uname: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub role : UserRole,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role : Option<UserRole>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
