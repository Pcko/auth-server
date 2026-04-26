use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub app_session_token: String,
    pub user: TokenUser,
}

#[derive(Debug, Serialize)]
pub struct TokenUser {
    pub id: Uuid,
    pub email: String,
}