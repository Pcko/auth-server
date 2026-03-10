use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
    pub is_dev: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let server_addr = std::env::var("SERVER_ADDR").context("SERVER_ADDR must be set")?;
        
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let is_dev = std::env::var("IS_DEV")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()?;

        Ok(Self {
            server_addr,
            database_url,
            is_dev,
        })
    }
}
