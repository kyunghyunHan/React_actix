// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        user_id -> Varchar,
        user_pw -> Varchar,
        user_name -> Varchar,
        user_phone -> Varchar,
    }
}
