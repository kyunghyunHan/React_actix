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
    pub user_pw: String,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub user_pw: &'a str,
    pub user_name: &'a str,
    pub user_phone: &'a str,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub user_id: String,
    pub user_pw: String,
    pub user_name: String,
    pub user_phone: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    pub user_pw: String,
}
pub fn create_post<'a>(
    conn: &MysqlConnection,
    user_id: &'a str,
    user_pw: &'a str,
    user_name: &'a str,
    user_phone: &'a str,
) -> String {
    let new_post = NewUser {
        user_id,
        user_pw,
        user_name,
        user_phone,
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
