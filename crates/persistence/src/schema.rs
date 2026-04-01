// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        uid -> Uuid,
        token_hash -> Text,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        revoked_at -> Nullable<Timestamptz>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    user (id) {
        id -> Uuid,
        email -> Text,
        name -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_allowed -> Bool,
        is_mfa_enabled -> Bool,
        role -> UserRole,
    }
}

diesel::joinable!(sessions -> user (uid));

diesel::allow_tables_to_appear_in_same_query!(sessions, user,);
