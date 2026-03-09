use crate::routes::{auth, health, user};
use crate::state::AppState;
use axum::Router;
use tower_cookies::CookieManagerLayer;

pub fn app(state: AppState) -> Router {
    Router::new()
        .nest("/users", user::router())
        .nest("/auth", auth::router())
        .merge(health::router())
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
