use crate::services::user_service::UserError::Repo;
use domain::model::user::{User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::str::FromStr;
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

    pub async fn get_user(&self, id: u64) -> Result<User, UserError> {
        let uid = Uuid::from_str(id.to_string().as_str())
            .map_err(|_| UserError::Unexpected("User ID could not be parsed".to_string()))?;

        let user = self
            .repo
            .find_by_id(UserId::new(uid))
            .await
            .map_err(|err| UserError::Repo(err))?;

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
