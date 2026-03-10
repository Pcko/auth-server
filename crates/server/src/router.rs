use crate::middleware::auth_session::AuthSession;
use crate::routes::{auth, health, user};
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::middleware::FromExtractor;
use axum::{Router, middleware};
use tower_cookies::CookieManagerLayer;

pub fn app(state: AppState) -> Router {
    let users = user::router()
        .route_layer(middleware::from_extractor_with_state::<AuthSession, _>(
            state.clone(),
        ));

    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users)
        .merge(health::router())
        .layer(CookieManagerLayer::new())
        .with_state(state)
}