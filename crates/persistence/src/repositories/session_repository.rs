use crate::models::session_row::{NewSessionRow, SessionRow};
use crate::schema;
use crate::schema::sessions::dsl::{sessions, token_hash, uid};
use diesel::dsl::update;

use crate::schema::sessions::{expires_at, id, last_seen_at};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use domain::model::session::{NewSession, Session, SessionId};
use domain::model::user::UserId;
use domain::repositories::session_repository::{SessionRepository, SessionRepositoryError};
use time::OffsetDateTime;
use tracing::error;

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
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let row = sessions
            .find(given_id.as_uuid())
            .first::<SessionRow>(&mut conn)
            .await
            .optional()
            .map_err(map_diesel_error)?;

        Ok(row.map(Into::into))
    }

    async fn find_by_uid(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Session>, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let rows = sessions
            .filter(uid.eq(user_id.as_uuid()))
            .load::<SessionRow>(&mut conn)
            .await
            .map_err(map_diesel_error)?;
        
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn insert(&self, session: NewSession) -> Result<Session, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let new_row = NewSessionRow::from(&session);

        let created_row = diesel::insert_into(schema::sessions::table)
            .values(&new_row)
            .returning(SessionRow::as_returning())
            .get_result::<SessionRow>(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        Ok(created_row.into())
    }

    async fn delete_by_id(&self, session_id: SessionId) -> Result<(), SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let deleted = diesel::delete(sessions.find(session_id.as_uuid()))
            .execute(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        if deleted == 0 {
            return Err(SessionRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn delete_by_uid(&self, user_id: UserId) -> Result<(), SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let deleted = diesel::delete(sessions.filter(uid.eq(user_id.as_uuid())))
            .execute(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        if deleted == 0 {
            return Err(SessionRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn delete_by_token_hash(
        &self,
        given_token_hash: String,
    ) -> Result<(), SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        diesel::delete(sessions.filter(token_hash.eq(given_token_hash)))
            .execute(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        Ok(())
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
            .first::<SessionRow>(&mut conn)
            .await
            .optional()
            .map_err(map_diesel_error)?;

        Ok(row.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Session>, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let rows = sessions
            .load::<SessionRow>(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        return Ok(rows.into_iter().map(Into::into).collect());
    }

    async fn update_refresh_token_data(
        &self,
        session: Session,
    ) -> Result<Session, SessionRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        let updated_row = update(sessions.find(session.id.as_uuid()))
            .set((
                token_hash.eq(session.token_hash),
                expires_at.eq(session.expires_at),
                last_seen_at.eq(OffsetDateTime::now_utc()),
            ))
            .get_result::<SessionRow>(&mut conn)
            .await
            .map_err(|e| SessionRepositoryError::Unexpected(e.to_string()))?;

        Ok(updated_row.into())
    }
}

/**
This function serves as Error translator so UserRepositoryError will be thrown
*/
fn map_diesel_error(err: DieselError) -> SessionRepositoryError {
    error!("diesel session error: {err}");
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            SessionRepositoryError::Conflict
        }
        other => SessionRepositoryError::Unexpected(other.to_string()),
    }
}
