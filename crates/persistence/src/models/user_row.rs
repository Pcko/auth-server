use crate::schema::user;
use diesel::{Insertable, Queryable, Selectable};
use domain::model::user::{NewUser, User, UserId};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user)]
pub struct NewUserRow<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> User {
        User {
            uid: UserId::new(row.id),
            uname: row.name,
            umail: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at,
        }
    }
}

impl<'a> From<&'a NewUser> for NewUserRow<'a> {
    fn from(usr: &'a NewUser) -> NewUserRow<'a> {
        Self {
            name: &usr.name,
            email: &usr.email,
            password_hash: &usr.password_hash,
        }
    }
}
