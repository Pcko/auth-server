mod logging;

use anyhow::Result;
use application::auth_service::AuthService;
use application::user_service::UserService;
use diesel::connection::set_default_instrumentation;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, bb8};
use persistence::repositories::postgres_user_repository::DieselUserRepository;
use server::router::app;
use server::state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // ENV vars
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");

    // DB Init
    set_default_instrumentation(logging::diesel_logger)
        .expect("failed to set default logging instance");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().build(config).await?;

    // Webserver Init
    let state = build_app(pool);
    let app = app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn build_app(pool: Pool<AsyncPgConnection>) -> AppState {
    // Arc = smart pointer, so basically auth service and user service reference the same user repo
    let user_repo = Arc::new(DieselUserRepository::new(pool.clone()));
    // we only clone the Arc it still points to the same repo
    let auth_service = Arc::new(AuthService::new(user_repo.clone()));
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    AppState {
        pool: pool,
        auth_service: auth_service,
        user_service: user_service,
    }
}
