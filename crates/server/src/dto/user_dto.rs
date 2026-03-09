use diesel::internal::derives::multiconnection::time::OffsetDateTime;
use domain::model::user::User;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
}

impl From<User> for UserResponseDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.uid.as_uuid(),
            username: user.uname,
            email: user.umail,
            created_at: user.created_at,
        }
    }
}
