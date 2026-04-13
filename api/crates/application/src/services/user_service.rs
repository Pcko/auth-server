use crate::services::auth_service::AuthService;
use domain::model::user::{UpdateUserCommand, UpdatedUser, User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
    auth_service: Arc<AuthService>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>, auth_service: Arc<AuthService>) -> Self {
        Self { repo, auth_service }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        let result = self.repo.find_all().await.map_err(UserError::Repo)?;

        Ok(result)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, UserError> {
        let user = self
            .repo
            .find_by_id(UserId::new(id))
            .await
            .map_err(UserError::Repo)?;

        user.ok_or(UserError::NotFound)
    }

    pub async fn update_user(
        &self,
        existing: User,
        cmd: UpdateUserCommand,
    ) -> Result<User, UserError> {
        if cmd.username.is_none()
            && cmd.email.is_none()
            && cmd.new_password.is_none()
            && cmd.mfa.is_none()
        {
            return Err(UserError::Validation("no fields to update".into()));
        }

        let name = match cmd.username {
            Some(name) => {
                if name.trim().is_empty() {
                    return Err(UserError::Validation("username is empty".into()));
                }
                name
            }
            None => existing.uname,
        };

        let email = match cmd.email {
            Some(email) => {
                if email.trim().is_empty() {
                    return Err(UserError::Validation("email is empty".into()));
                }
                email
            }
            None => existing.email,
        };

        let password_hash = match cmd.new_password {
            Some(password) => {
                if password.trim().is_empty() {
                    return Err(UserError::Validation("password is empty".into()));
                }

                self.auth_service
                    .hash_password(&password)
                    .map_err(|_| UserError::Validation("password is invalid".into()))?
            }
            None => existing.password_hash,
        };

        //TODO handle mfa activation
        let mfa = match cmd.mfa {
            Some(mfa) => mfa,
            None => existing.mfa
        };

        let updated_usr = UpdatedUser {
            id: existing.uid,
            name,
            email,
            password_hash,
            is_mfa_enabled: mfa,
        };

        self.repo.update(updated_usr).await.map_err(UserError::Repo)
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("repository error: {0}")]
    Repo(#[from] UserRepositoryError),
    #[error("repository error: {0}")]
    Unexpected(String),
    #[error("validation error: {0}")]
    Validation(String),
}
