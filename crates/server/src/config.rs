use anyhow::{Context, Result};
use tracing::level_filters::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
    pub is_dev: bool,
    pub session_secret: Vec<u8>,
    pub refresh_secret: Vec<u8>,
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
        let session_secret =
            std::env::var("SESSION_SECRET").context("SESSION_SECRET must be set")?;
        let session_secret = session_secret.as_bytes().to_vec();

        // Server Side key for refresh token hashing
        let refresh_secret =
            std::env::var("REFRESH_SECRET").context("REFRESH_SECRET must be set")?;
        let refresh_secret = refresh_secret.as_bytes().to_vec();

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .parse::<LevelFilter>()?;

        Ok(Self {
            server_addr,
            database_url,
            is_dev,
            session_secret,
            refresh_secret,
            log_level,
        })
    }
}
