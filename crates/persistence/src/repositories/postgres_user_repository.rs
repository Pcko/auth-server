use crate::models::user_row::UserRow;
use crate::schema;
use crate::schema::user::dsl::*;
use diesel::QueryDsl;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{ExpressionMethods, OptionalExtension, SelectableHelper};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use domain::model::user::{User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Pool<AsyncPgConnection>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, user_id: UserId) -> Result<Option<User>, UserRepositoryError> {
        let mut conn = self.pool.get()
            .await.map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        let row = user
            .filter(id.eq(user_id.as_uuid()))
            .select(UserRow::as_select())
            .first::<UserRow>(&mut conn)
            .await
            .optional()
            .map_err(map_diesel_error)?;

        Ok(row.map(Into::into))
    }

    async fn find_by_email(&self, user_email: &str) -> Result<Option<User>, UserRepositoryError> {
        todo!()
    }

    async fn save(&self, given_user: &User) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let mut conn = self.pool.get()
            .await.map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        let rows = schema::user::table
            .select(UserRow::as_select())
            .load::<UserRow>(&mut conn)
            .await
            .map_err(map_diesel_error);

        let mut users : Vec<User> = Vec::new();
        for row in rows? {
            users.push(row.into())
        }

        Ok(users)
    }
}

fn map_diesel_error(err: DieselError) -> UserRepositoryError {
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            UserRepositoryError::Conflict
        }
        other => UserRepositoryError::Unexpected(other.to_string()),
    }
}
