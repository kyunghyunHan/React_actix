// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
    }
}
