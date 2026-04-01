use domain::model::user::User;
use domain::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
}

impl AdminService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn elevate_user(&self, user: User) -> Result<User, AdminError> {
        self.user_repo
            .update(user)
            .await
            .map_err(|_| AdminError::Unexpected)
    }
}

pub enum AdminError {
    Unexpected,
}
