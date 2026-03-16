use crate::middleware::request_info_extractor::ExtractRequestInfo;
use crate::middleware::user_extractor::UserExtractor;
use crate::routes::{auth, health, user};
use crate::state::AppState;
use axum::{Router, http, middleware};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;

pub fn app(state: AppState) -> Router {
    // infrastructure layer
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

    // feature specific
    let users = user::router().route_layer(
        ServiceBuilder::new()
            .layer(middleware::from_extractor_with_state::<ExtractRequestInfo, _>(state.clone()))
            .layer(middleware::from_extractor_with_state::<UserExtractor, _>(
                state.clone(),
            )),
    );

    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users)
        .merge(health::router())
        .layer(infra_layer)
        .with_state(state)
}
