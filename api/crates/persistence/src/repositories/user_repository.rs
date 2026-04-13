use crate::models::user_row::{NewUserRow, UpdateUserChanges, UserRow};
use crate::schema;
use crate::schema::user::dsl::*;
use diesel::QueryDsl;
use diesel::pg::Pg;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{ExpressionMethods, OptionalExtension, SelectableHelper, debug_query};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use domain::model::user::{NewUser, UpdatedUser, User, UserId};
use domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use tracing::{error, info, instrument};

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

    #[instrument(name = "user_repo.save", skip(self, given_user), fields(email = %given_user.email, username = %given_user.name))]
    async fn save(&self, given_user: &NewUser) -> Result<User, UserRepositoryError> {
        info!("save: acquiring db connection");
        let mut conn = self.pool.get().await.map_err(|e| {
            error!(error = %e, "save: failed to acquire db connection");
            UserRepositoryError::Unexpected(e.to_string())
        })?;
        info!("save: db connection acquired");

        let new_row: NewUserRow = given_user.into();

        info!("save: executing insert");
        let created_user_row = diesel::insert_into(schema::user::table)
            .values(&new_row)
            .returning(UserRow::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                error!(error = ?e, "save: insert failed");
                map_diesel_error(e)
            })?;
        info!("save: insert complete");

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

    async fn update(&self, updated_user: UpdatedUser) -> Result<User, UserRepositoryError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        let patch: UpdateUserChanges = updated_user.into();
        let updated_usr = diesel::update(user.find(patch.id))
            .set(patch)
            .returning(UserRow::as_returning())
            .get_result::<UserRow>(&mut conn)
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        Ok(updated_usr.into())
    }
}

/**
   This function serves as Error translator so UserRepositoryError will be thrown
*/
fn map_diesel_error(err: DieselError) -> UserRepositoryError {
    error!("diesel user error: {err}");
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            UserRepositoryError::Conflict
        }
        other => UserRepositoryError::Unexpected(other.to_string()),
    }
}
