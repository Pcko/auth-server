use crate::models::user_row::{NewUserRow, UserRow};
use crate::schema;
use crate::schema::user::dsl::*;
use diesel::pg::Pg;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{ExpressionMethods, OptionalExtension, SelectableHelper, debug_query};
use diesel::{Insertable, QueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use domain::model::user::{NewUser, User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};

#[derive(Clone)]
pub struct DieselUserRepository {
    pool: Pool<AsyncPgConnection>,
}

impl DieselUserRepository {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for DieselUserRepository {
    async fn find_by_id(&self, user_id: UserId) -> Result<Option<User>, UserRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

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
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        let query = user
            .filter(email.eq(user_email))
            .select(UserRow::as_select());

        println!("SQL: {}", debug_query::<Pg, _>(&query));

        let user_row = query
            .first::<UserRow>(&mut conn)
            .await
            .optional()
            .map_err(map_diesel_error)?;

        Ok(user_row.map(Into::into))
    }

    async fn exists_by_email(&self, user_email: &str) -> Result<bool, UserRepositoryError> {
        todo!()
    }

    async fn save(&self, given_user: &NewUser) -> Result<User, UserRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        // Convert Domain type NewUser to NewUserRow (the specific persistence type)
        let new_row: NewUserRow = given_user.into();
        let created_user_row = diesel::insert_into(schema::user::table)
            .values(&new_row)
            .returning(UserRow::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(map_diesel_error)?;

        // convert UserRow to User and return it
        Ok(created_user_row.into())
    }

    async fn find_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        // select every user out of the db
        let rows = schema::user::table
            .select(UserRow::as_select())
            .load::<UserRow>(&mut conn)
            .await
            .map_err(map_diesel_error);

        // go over each selected row and add it to a new vector
        let mut users: Vec<User> = Vec::new();
        for row in rows? {
            users.push(row.into())
        }

        Ok(users)
    }
}

/**
   This function serves as Error translator so UserRepositoryError will be thrown
*/
fn map_diesel_error(err: DieselError) -> UserRepositoryError {
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            UserRepositoryError::Conflict
        }
        other => UserRepositoryError::Unexpected(other.to_string()),
    }
}
