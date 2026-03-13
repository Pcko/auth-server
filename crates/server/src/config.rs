use anyhow::{Context, Result};
use tracing::level_filters::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
    pub is_dev: bool,
    pub secret_key: Vec<u8>,
    pub log_level: LevelFilter,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let server_addr = std::env::var("SERVER_ADDR").context("SERVER_ADDR must be set")?;

        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        // Dev env to evade security features
        let is_dev = std::env::var("IS_DEV")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()?;

        // Server Side key for session token hashing
        let secret_key = std::env::var("SECRET_KEY").context("SECRET_KEY must be set")?;
        let secret_key = secret_key.as_bytes().to_vec();

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .parse::<LevelFilter>()?;

        Ok(Self {
            server_addr,
            database_url,
            is_dev,
            secret_key,
            log_level,
        })
    }
}
