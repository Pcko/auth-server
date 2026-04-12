use domain::model::user::{UpdateUserCommand, User};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserResponseDTO {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub mfa_enabled: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mfa_enabled: Option<bool>,
}

impl From<User> for UserResponseDTO {
    fn from(user: User) -> Self {
        let created_at = user
            .created_at
            .format(&Rfc3339)
            .unwrap_or_else(|_| user.created_at.to_string());

        Self {
            id: user.uid.as_uuid().to_string(),
            username: user.uname,
            email: user.email,
            created_at,
            mfa_enabled: user.mfa
        }
    }
}

impl UpdateUserRequest {
    pub fn into_command(self) -> UpdateUserCommand {
        UpdateUserCommand {
            username: self.username,
            email: self.email,
            new_password: self.password,
            mfa: self.mfa_enabled,
        }
    }
}
