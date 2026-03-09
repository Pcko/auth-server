use crate::models::session_row::{NewSessionRow, SessionRow};
use crate::schema;
use crate::schema::sessions::dsl::{sessions, token_hash};

use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use domain::model::session::{NewSession, Session, SessionId};
use domain::model::user::UserId;
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};

#[derive(Clone)]
pub struct DieselSessionRepository {
    pool: Pool<AsyncPgConnection>,
}

impl DieselSessionRepository {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionRepository for DieselSessionRepository {
    async fn find_by_id(
        &self,
        given_id: SessionId,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        todo!()
    }

    async fn find_by_uid(&self, uid: UserId) -> Result<Option<Session>, SessionRepositoryError> {
        todo!()
    }

    async fn insert(&self, session: NewSession) -> Result<Session, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let new_row = NewSessionRow::from(&session);
        let created_session_row = diesel::insert_into(schema::sessions::table)
            .values(&new_row)
            .returning(SessionRow::as_returning())
            .get_result::<SessionRow>(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        eprintln!("SESSION ROW OK: {:?}", created_session_row);
        let result: Session = created_session_row.into();
        eprintln!("SESSION CONVERSION OK: {:?}", session.uid);
        Ok(result)
    }

    async fn delete_by_id(&self, session_id: SessionId) -> Result<(), SessionRepositoryError> {
        todo!()
    }

    async fn delete_by_uid(&self, uid: UserId) -> Result<(), SessionRepositoryError> {
        todo!()
    }

    async fn find_by_token_hash(
        &self,
        given_token_hash: String,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let row = sessions
            .filter(token_hash.eq(given_token_hash))
            .select(SessionRow::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map_err(map_diesel_error)?;

        Ok(row.map(Into::into))
    }
}

/**
This function serves as Error translator so UserRepositoryError will be thrown
*/
fn map_diesel_error(err: DieselError) -> SessionRepositoryError {
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            SessionRepositoryError::Conflict
        }
        other => SessionRepositoryError::Unexpected(other.to_string()),
    }
}
