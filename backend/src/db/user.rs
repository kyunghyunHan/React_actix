use crate::db;
use actix_web::Error;
use chrono::{NaiveDateTime, Utc};
use db::connection;
use db::schema::users;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}

pub fn create_post<'a>(
    conn: &MysqlConnection,
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
) -> String {
    let new_post = NewUser {
        first_name,
        last_name,
        email,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .execute(&*conn)
        .unwrap();
    format!("hey")
}
