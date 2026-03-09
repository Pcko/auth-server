use application::services::auth_service::AuthService;
use std::sync::Arc;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use application::services::user_service::UserService;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool : Pool<AsyncPgConnection>,
    pub config : AppConfig,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}
