#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::schema::users;
use actix_web::Error;
use diesel::prelude::*;
use diesel::MysqlConnection;

use super::connection;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

// pub fn create_post<'a>(
//     conn: &MysqlConnection,
//     username: &'a str,
//     email: &'a str,
//     password: &'a str,
//     login_session: &'a str,
// ) -> String {
//     let new_post = UserDTO {
//         username,
//         email,
//         password,
//         login_session,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_post)
//         .execute(conn)
//         .unwrap();
//     format!("hey")
// }
