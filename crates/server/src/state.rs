use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<AsyncPgConnection>,
}
