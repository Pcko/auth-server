use crate::state::AppState;
use axum::{Router, routing::get};

async fn health_msg() -> &'static str {
    "healthy"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_msg))
}
