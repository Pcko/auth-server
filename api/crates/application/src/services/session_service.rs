use domain::model::session::{Session, SessionId};
use domain::model::user::UserId;
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub struct SessionService {
    session_repo: Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(session_repo: Arc<dyn SessionRepository>) -> Self {
        Self { session_repo }
    }

    pub async fn get_by_session_id(&self, sid: Uuid) -> Result<Option<Session>, SessionError> {
        self.session_repo
            .find_by_id(SessionId::new(sid))
            .await
            .map_err(SessionError::from)
    }

    pub async fn get_all(&self) -> Result<Vec<Session>, SessionError> {
        self.session_repo
            .find_all()
            .await
            .map_err(SessionError::from)
    }

    pub async fn delete(&self, sid: Uuid) -> Result<(), SessionError> {
        self.session_repo
            .delete_by_id(SessionId::new(sid))
            .await
            .map_err(SessionError::from)
    }

    pub async fn delete_by_uid(&self, uid: Uuid) -> Result<(), SessionError> {
        self.session_repo
            .delete_by_uid(UserId::new(uid))
            .await
            .map_err(SessionError::from)
    }

    pub async fn get_sessions_by_uid(&self, uid: Uuid) -> Result<Vec<Session>, SessionError> {
        self.session_repo
            .find_by_uid(UserId::new(uid))
            .await
            .map_err(SessionError::from)
    }
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("session not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("repository error: {0}")]
    SessionRepo(SessionRepositoryError),
    #[error("unexpected")]
    Unexpected,
}

impl From<SessionRepositoryError> for SessionError {
    fn from(error: SessionRepositoryError) -> Self {
        match error {
            SessionRepositoryError::NotFound => SessionError::NotFound,
            SessionRepositoryError::Conflict => SessionError::SessionRepo(error),
            SessionRepositoryError::Unexpected(_) => SessionError::SessionRepo(error),
        }
    }
}
