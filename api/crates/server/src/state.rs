use crate::config::AppConfig;
use application::services::admin_service::AdminService;
use application::services::auth_service::AuthService;
use application::services::session_service::SessionService;
use application::services::user_service::UserService;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<AsyncPgConnection>,
    pub config: AppConfig,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
    pub session_service: Arc<SessionService>,
    pub admin_service: Arc<AdminService>,
}
