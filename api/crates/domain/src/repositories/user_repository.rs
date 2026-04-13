use crate::model::user::{NewUser, UpdatedUser, User, UserId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User does not exist")]
    NotFound,
    #[error("User already exists")]
    Conflict,
    #[error("Unexpected: {0}")]
    Unexpected(String),
}

/**
   Generic UserRepository
*/
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: &NewUser) -> Result<User, UserRepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError>;
    async fn update(&self, user: UpdatedUser) -> Result<User, UserRepositoryError>;
}
