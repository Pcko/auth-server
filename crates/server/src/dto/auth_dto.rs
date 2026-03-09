use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseDTO {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutDTO{
    pub session_id: String,
}