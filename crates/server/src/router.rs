use crate::routes::{auth, health, user};
use crate::state::AppState;
use axum::Router;

pub fn app(state: AppState) -> Router {
    Router::new()
        .nest("/users", user::router())
        .nest("/auth", auth::router())
        .merge(health::router())
        .with_state(state)
}
