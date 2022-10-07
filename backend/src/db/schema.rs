// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}
