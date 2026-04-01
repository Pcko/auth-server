use domain::model::user::{User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        let result = self.repo.find_all().await;
        Ok(result?)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, UserError> {
        let user = self
            .repo
            .find_by_id(UserId::new(id))
            .await
            .map_err(UserError::Repo)?;

        Ok(user.unwrap())
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("repository error: {0}")]
    Repo(#[from] UserRepositoryError),
    #[error("repository error: {0}")]
    Unexpected(String),
}
