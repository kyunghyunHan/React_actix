// @generated automatically by Diesel CLI.

diesel::table! {
    resumes (id) {
        id -> Integer,
        cv_address -> Varchar,
        cv_email -> Varchar,
        cv_letter -> Varchar,
        cv_tech -> Varchar,
        cv_edu -> Varchar,
        cv_cert -> Varchar,
        cv_awards -> Varchar,
        cv_project -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_id -> Varchar,
        user_pw -> Varchar,
        user_name -> Varchar,
        user_phone -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    resumes,
    users,
);
