// use crate::db::user::users::dsl::users;
use crate::db::{self};

// use chrono::{NaiveDateTime, Utc};
// use db::connection::DbPool;
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
// pub fn insert_new_user(db: &MysqlConnection, user: CreateUser) -> Result<User, Error> {
//     use self::users::dsl::*;

//     // Create insertion model
//     // let uuid = format!("{}", uuid::Uuid::new_v4());
//     let new_user = NewUser {
//         first_name: &user.first_name,
//         last_name: &user.last_name,
//         email: &user.email,
//     };

//     // normal diesel operations
//     diesel::insert_into(users)
//         .values(&new_user)
//         .execute(&self.0)
//         .expect("Error inserting person");

//     let mut items = users
//         .filter(id.eq(&uuid))
//         .load::<User>(&self.0)
//         .expect("Error loading person");

//     Ok(items.pop().unwrap())
// }
