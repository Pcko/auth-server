use crate::middleware::auth_session::AuthSession;
use crate::routes::{auth, health, user};
use crate::state::AppState;
use axum::handler::Handler;
use axum::{Router, http, middleware};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;

pub fn app(state: AppState) -> Router {
    let auth_layer = middleware::from_extractor_with_state::<AuthSession, _>(state.clone());

    let users = user::router().route_layer(auth_layer);

    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users)
        .merge(health::router())
        .layer(
            ServiceBuilder::new()
                // Hide secret header values so they don't get logged or exposed.
                .layer(SetSensitiveHeadersLayer::new([
                    http::header::AUTHORIZATION,
                    http::header::COOKIE,
                    http::header::SET_COOKIE,
                ]))
                // Add an x-request-id to the request if it doesn't already have one.
                .layer(SetRequestIdLayer::x_request_id(
                    tower_http::request_id::MakeRequestUuid,
                ))
                // Put that same request id on the response too.
                .layer(PropagateRequestIdLayer::x_request_id())
                // Just so cookies can be handled in routes
                .layer(CookieManagerLayer::new())
                // Log basic info about the HTTP request/response.
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(state)
}
