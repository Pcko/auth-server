mod diesel_logging;

use anyhow::Result;
use application::services::admin_service::AdminService;
use application::services::auth_service::AuthService;
use application::services::session_service::SessionService;
use application::services::token_service::TokenService;
use application::services::user_service::UserService;
use diesel::connection::set_default_instrumentation;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use persistence::repositories::session_repository::DieselSessionRepository;
use persistence::repositories::user_repository::DieselUserRepository;
use server::config::AppConfig;
use server::router::app;
use server::state::AppState;
use std::sync::Arc;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Provider that reads the env vars and creates a config
    let config = AppConfig::from_env()?;

    // tracing logging init
    let filter = EnvFilter::builder()
        .with_default_directive(config.log_level.into())
        .from_env_lossy();
    init_tracing(filter);
    // DB init
    set_default_instrumentation(diesel_logging::diesel_logger)
        .expect("failed to set default logging instance");
    let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
    let pool = Pool::builder().build(db_config).await?;
    info!("Connected to DB!");

    // Webserver init
    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    let state = build_app(pool, config);
    let app = app(state);

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn build_app(pool: Pool<AsyncPgConnection>, config: AppConfig) -> AppState {
    // Arc = atomic(thread safe) smart pointer, so basically auth service and user service reference the same user repo
    // Repos
    let user_repo = Arc::new(DieselUserRepository::new(pool.clone()));
    let session_repo = Arc::new(DieselSessionRepository::new(pool.clone()));
    // Services
    let session_service = Arc::new(SessionService::new(session_repo.clone()));
    let token_service = Arc::new(TokenService::new(session_repo.clone()));
    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        session_repo.clone(),
        token_service.clone(),
    ));
    let user_service = Arc::new(UserService::new(user_repo.clone()));
    let admin_service = Arc::new(AdminService::new(user_repo.clone()));

    AppState {
        pool: pool,
        auth_service: auth_service,
        user_service: user_service,
        config: config,
        session_service: session_service,
        admin_service: admin_service,
    }
}

fn init_tracing(filter: EnvFilter) {
    // persist logs per day
    let file_appender = rolling::daily("logs", "idp.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // init the tracing subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .flatten_event(true)
                .with_target(true)
                .with_writer(non_blocking),
        )
        // pretty should be replaced with compact tbh (less readable but more efficient)
        .with(tracing_subscriber::fmt::layer().pretty().with_target(false))
        .init()
}
