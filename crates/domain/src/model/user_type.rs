use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum UserType {
    Admin,
    Normal,
}

impl UserType {
    pub fn as_str(&self) -> &str {
        match self {
            UserType::Admin => "admin",
            UserType::Normal => "normal",
        }
    }
}

impl TryFrom<&str> for UserType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(UserType::Admin),
            "normal" => Ok(UserType::Normal),
            _ => Err("Unknown user type"),
        }
    }
}
