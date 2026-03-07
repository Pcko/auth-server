// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        name -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
    }
}
