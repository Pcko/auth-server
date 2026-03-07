use crate::model::user::{User, UserId};

#[derive(Debug)]
pub enum UserRepositoryError {
    NotFound,
    Conflict,
    Unexpected(String),
}

/**
    Generic UserRepository for Postgres and Redis
 */
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError>;
}