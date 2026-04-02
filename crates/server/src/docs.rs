use aide::openapi::{Info, OpenApi};
use aide::scalar::Scalar;
use axum::Extension;
use axum::Json;
use axum::Router;
use axum::response::Html;
use axum::routing::get;

pub fn openapi() -> OpenApi {
    OpenApi {
        info: Info {
            title: "Central Auth Server API".to_string(),
            description: Some(
                "HTTP API for authentication, session management, and administrative user operations."
                    .to_string(),
            ),
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Info::default()
        },
        ..OpenApi::default()
    }
}

pub async fn serve_api(Extension(api): Extension<OpenApi>) -> Json<OpenApi> {
    Json(api)
}

async fn scalar_docs() -> Html<String> {
    Html(
        Scalar::new("/openapi.json")
            .with_title("Central Auth Server API")
            .html(),
    )
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/openapi.json", get(serve_api))
        .route("/docs", get(scalar_docs))
}
