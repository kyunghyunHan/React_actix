//이력서
use crate::db::{self};

use db::schema::resumes;
use diesel::query_dsl::RunQueryDsl;
use diesel::MysqlConnection;

use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable)]
pub struct Resume {
    pub id: i32,
    pub cv_address: String,
    pub cv_email: String,
    pub cv_letter: String,
    pub cv_tech: String,
    pub cv_edu: String,
    pub cv_cert: String,
    pub cv_awards: String,
    pub cv_project: String,
    pub user_resumekey: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "resumes"]
pub struct NewResume<'a> {
    pub cv_address: &'a str,
    pub cv_email: &'a str,
    pub cv_letter: &'a str,
    // pub cv_tech: &'a str,
    pub cv_edu: &'a str,
    pub cv_cert: &'a str,
    pub cv_awards: &'a str,
    pub cv_project: &'a str,
    pub cv_user_key: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Resumes {
    pub cv_address: String,
    pub cv_email: String,
    pub cv_letter: String,
    // pub cv_tech: String,
    pub cv_edu: String,
    pub cv_cert: String,
    pub cv_awards: String,
    pub cv_project: String,
    pub cv_user_key: i32,
}
pub fn create_resume<'a>(
    conn: &MysqlConnection,
    cv_address: &'a str,
    cv_email: &'a str,
    cv_letter: &'a str,
    // cv_tech: &'a str,
    cv_edu: &'a str,
    cv_cert: &'a str,
    cv_awards: &'a str,
    cv_project: &'a str,
    cv_user_key: i32,
) -> String {
    let new_post = NewResume {
        cv_address: cv_address,
        cv_email: cv_email,
        cv_letter: cv_letter,
        // cv_tech: cv_tech,
        cv_edu: cv_edu,
        cv_cert: cv_cert,
        cv_awards: cv_awards,
        cv_project: cv_project,
        cv_user_key: cv_user_key,
    };

    diesel::insert_into(resumes::table)
        .values(&new_post)
        .execute(&*conn)
        .unwrap();
    format!("hey")
}
