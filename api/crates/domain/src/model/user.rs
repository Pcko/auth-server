use crate::model::user_type::UserRole;
use std::fmt;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct User {
    pub uid: UserId,
    pub uname: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub role: UserRole,
    pub mfa: bool,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: Option<UserRole>,
}

// For Application Layer should move away
pub struct UpdateUserCommand {
    pub username: Option<String>,
    pub email: Option<String>,
    pub new_password: Option<String>,
    pub mfa: Option<bool>,
}

pub struct UpdatedUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub is_mfa_enabled: bool,
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
