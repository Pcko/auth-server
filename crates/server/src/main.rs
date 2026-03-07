use anyhow::Result;
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;
use server::router::app;
use server::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // ENV vars
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");

    // DB Init
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await?;

    // Webserver Init
    let state = AppState { db: pool };
    let app = app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
