use domain::model::user::User;
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserRepositoryError> {
        let result = self.repo.find_all().await;
        Ok(result?)
    }
}
