//이력서
use crate::db::{self};

use db::schema::techs;
use diesel::query_dsl::RunQueryDsl;
use diesel::MysqlConnection;

use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable)]
pub struct Tech {
    pub id: i32,
    pub tc_list: String,
    pub tc_user_key: i32,
    pub tc_resume_key: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "techs"]
pub struct NewTech<'a> {
    pub tc_list: &'a str,
    pub tc_user_key: i32,
    pub tc_resume_key: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Techs {
    pub tc_list: String,
    pub tc_user_key: i32,
    pub tc_resume_key: i32,
}
pub fn create_tech<'a>(
    conn: &MysqlConnection,
    tc_list: &'a str,
    tc_user_key: i32,
    tc_resume_key: i32,
) -> String {
    let new_post = NewTech {
        tc_list: tc_list,
        tc_user_key: tc_user_key,
        tc_resume_key: tc_resume_key,
    };

    diesel::insert_into(techs::table)
        .values(&new_post)
        .execute(&*conn)
        .unwrap();
    format!("hey")
}
