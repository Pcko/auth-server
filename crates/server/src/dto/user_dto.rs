use domain::model::user::User;
use schemars::JsonSchema;
use serde::Serialize;
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserResponseDTO {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
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
        }
    }
}
