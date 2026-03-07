use diesel::{Insertable, Queryable, Selectable};
use domain::model::user::{User, UserId};
use uuid::Uuid;
use crate::schema::user;
use time::OffsetDateTime;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime
}

#[derive(Debug,Insertable)]
#[diesel(table_name = user)]
pub struct NewUserRow<'a>{
    pub id: Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl From<UserRow> for User{
    fn from(row: UserRow) -> User{
        User{
            uid: UserId::new(row.id),
            uname: row.name,
            umail: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at
        }
    }
}

impl<'a> From<&'a User> for NewUserRow<'a>{
    fn from(usr: &'a User) -> NewUserRow<'a>{
        Self{
            id : usr.uid.as_uuid(),
            name: &usr.uname,
            email: &usr.umail,
            password_hash: &usr.password_hash
        }
    }
}