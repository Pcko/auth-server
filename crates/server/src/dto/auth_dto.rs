use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct AuthMeResponseDTO {
    pub uid: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SuccessResponseDTO {
    pub success: bool,
}
