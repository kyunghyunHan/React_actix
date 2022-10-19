// @generated automatically by Diesel CLI.

diesel::table! {
    resumes (id) {
        id -> Integer,
        cv_address -> Varchar,
        cv_email -> Varchar,
        cv_letter -> Varchar,
        cv_edu -> Varchar,
        cv_cert -> Varchar,
        cv_awards -> Varchar,
        cv_project -> Varchar,
        cv_user_key -> Integer,
    }
}

diesel::table! {
    techs (id) {
        id -> Integer,
        tc_list -> Varchar,
        tc_user_key -> Integer,
        tc_resume_key -> Integer,
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

diesel::joinable!(resumes -> users (cv_user_key));
diesel::joinable!(techs -> resumes (tc_resume_key));
diesel::joinable!(techs -> users (tc_user_key));

diesel::allow_tables_to_appear_in_same_query!(
    resumes,
    techs,
    users,
);
