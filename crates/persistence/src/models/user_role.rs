use diesel_derive_enum::DbEnum;
use domain::model::user_type::UserRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRoleDB {
    Admin,
    Normal,
}

impl From<UserRole> for UserRoleDB {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => UserRoleDB::Admin,
            UserRole::Normal => UserRoleDB::Normal,
        }
    }
}

impl UserRoleDB {
    pub fn into_domain(self) -> UserRole {
        match self {
            UserRoleDB::Admin => UserRole::Admin,
            UserRoleDB::Normal => UserRole::Normal,
        }
    }
}
