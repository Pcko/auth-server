use domain::model::user::User;
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserRepositoryError> {
        let result = self.repo.find_all().await;
        Ok(result?)
    }
}
