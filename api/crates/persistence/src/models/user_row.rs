use crate::models::user_role::UserRoleDB;
use crate::schema::user;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::model::user::{NewUser, UpdatedUser, User, UserId};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    #[diesel(skip_update)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub role: UserRoleDB,
    pub is_mfa_enabled: bool
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user)]
pub struct NewUserRow<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: Option<UserRoleDB>,
}

#[derive(AsChangeset)]
#[diesel(table_name = user)]
pub struct UpdateUserChanges {
    #[diesel(skip_update)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub is_mfa_enabled: bool,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> User {
        User {
            uid: UserId::new(row.id),
            uname: row.name,
            email: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at,
            role: row.role.into_domain(),
            mfa: row.is_mfa_enabled
        }
    }
}

impl Into<UserRow> for User {
    fn into(self) -> UserRow {
        UserRow {
            id: self.uid.as_uuid(),
            name: self.uname,
            email: self.email,
            password_hash: self.password_hash,
            created_at: self.created_at,
            role: self.role.into(),
            is_mfa_enabled: self.mfa
        }
    }
}

impl<'a> From<&'a NewUser> for NewUserRow<'a> {
    fn from(usr: &'a NewUser) -> NewUserRow<'a> {
        Self {
            name: &usr.name,
            email: &usr.email,
            password_hash: &usr.password_hash,
            role: usr.role.map(UserRoleDB::from),
        }
    }
}

impl From<UpdatedUser> for UpdateUserChanges {
    fn from(user: UpdatedUser) -> Self {
        Self {
            id: user.id.as_uuid(),
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            is_mfa_enabled: user.is_mfa_enabled,
        }
    }
}
