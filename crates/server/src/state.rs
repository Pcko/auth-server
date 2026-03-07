use application::auth_service::AuthService;
use std::sync::Arc;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use application::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub pool : Pool<AsyncPgConnection>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}
