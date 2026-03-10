use crate::model::session::{NewSession, Session, SessionId};
use crate::model::user::UserId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("Session does not exist")]
    NotFound,
    #[error("Conflict with session")]
    Conflict,
    #[error("unexpected: {0}")]
    Unexpected(String),
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, SessionRepositoryError>;
    async fn find_by_uid(&self, uid: UserId) -> Result<Option<Session>, SessionRepositoryError>;
    async fn insert(&self, session: NewSession) -> Result<Session, SessionRepositoryError>;
    async fn delete_by_id(&self, session_id: SessionId) -> Result<(), SessionRepositoryError>;
    async fn delete_by_uid(&self, uid: UserId) -> Result<(), SessionRepositoryError>;
    async fn delete_by_token_hash(&self, token_hash: String) -> Result<(), SessionRepositoryError>;
    async fn find_by_token_hash(
        &self,
        token_hash: String,
    ) -> Result<Option<Session>, SessionRepositoryError>;
  
}
