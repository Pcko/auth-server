use crate::state::AppState;
use aide::axum::ApiRouter;
use aide::axum::routing::get_with;

async fn health_msg() -> &'static str {
    "healthy"
}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route(
        "/health",
        get_with(health_msg, |op| {
            op.description("Get server status.")
                .response::<200, String>()
        }),
    )
}
