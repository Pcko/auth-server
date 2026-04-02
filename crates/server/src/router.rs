use crate::docs;
use crate::middleware::admin_extractor::AdminExtractor;
use crate::routes::{auth, health, session, user};
use crate::state::AppState;
use aide::axum::{ApiRouter, RouterExt};
use axum::{Extension, Router, http, middleware};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;

pub fn app(state: AppState) -> Router {
    let infra_layer = ServiceBuilder::new()
        .layer(SetSensitiveHeadersLayer::new([
            http::header::AUTHORIZATION,
            http::header::COOKIE,
            http::header::SET_COOKIE,
        ]))
        .layer(SetRequestIdLayer::x_request_id(
            tower_http::request_id::MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http());

    let admin_layer = ServiceBuilder::new()
        .layer(middleware::from_extractor_with_state::<AdminExtractor, _>(
            state.clone(),
        ));

    let admin_router = ApiRouter::new()
        .nest("/users", user::router())
        .nest("/sessions", session::admin_session_router())
        .layer(admin_layer);

    // main api router
    let api_router = ApiRouter::new()
        .nest("/auth", auth::router())
        .nest("/admin", admin_router)
        .nest("/sessions", session::user_session_router())
        .merge(health::router());

    // for OpenAPI docs
    let mut api = docs::openapi();

    api_router
        .finish_api(&mut api)
        .merge(docs::router())
        .layer(Extension(api))
        .layer(infra_layer)
        .with_state(state)
}
