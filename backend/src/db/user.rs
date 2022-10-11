use crate::db::{self};
use db::schema::users;
use diesel::query_dsl::RunQueryDsl;
use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub user_password: String,
    pub user_name: String,
    pub user_birth: String,
    pub user_address: String,
    pub user_email: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub user_password: &'a str,
    pub user_name: &'a str,
    pub user_birth: &'a str,
    pub user_address: &'a str,
    pub user_email: &'a str,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub user_id: String,
    pub user_password: String,
    pub user_name: String,
    pub user_birth: String,
    pub user_address: String,
    pub user_email: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    pub user_password: String,
}
pub fn create_post<'a>(
    conn: &MysqlConnection,
    user_id: &'a str,
    user_password: &'a str,
    user_name: &'a str,
    user_birth: &'a str,
    user_address: &'a str,
    user_email: &'a str,
) -> String {
    let new_post = NewUser {
        user_id,
        user_password,
        user_name,
        user_birth,
        user_address,
        user_email,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .execute(&*conn)
        .unwrap();
    format!("hey")
}
// pub fn get_all(conn: &MysqlConnection) -> Vec<User> {
//     use crate::db::schema::users::dsl::users;
//     users.load::<User>(conn).expect("Error loading posts")
// }
