use axum::Router;
use crate::state::AppState;
use crate::routes;

pub fn app(state: AppState) -> Router {
    let user_router = routes::user::router();
    let health_router = routes::health::router();

    Router::new()
        .nest("/users", user_router)
        .merge(health_router)
        .with_state(state)
}
