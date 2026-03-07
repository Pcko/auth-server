use axum::{routing::get, Router};
use crate::state::AppState;

async fn health_msg() -> &'static str {
    "healthy"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_msg))
}