use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
