// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Uuid,
        uid -> Uuid,
        jti -> Uuid,
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
    user (id) {
        id -> Uuid,
        email -> Text,
        name -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_allowed -> Bool,
        is_mfa_enabled -> Bool,
    }
}

diesel::joinable!(sessions -> user (uid));

diesel::allow_tables_to_appear_in_same_query!(sessions, user,);
