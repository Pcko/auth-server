use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RegisterDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
