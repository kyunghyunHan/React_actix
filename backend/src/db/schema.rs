// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        user_id -> Varchar,
        user_password -> Varchar,
        user_name -> Varchar,
        user_birth -> Varchar,
        user_address -> Varchar,
        user_email -> Varchar,
    }
}
