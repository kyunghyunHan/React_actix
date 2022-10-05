use crate::db::{
    connection::{establish_connection, init_pool, Conn, DbPool},
    user::create_post,
};

use actix_web::{get, post, web, HttpResponse, Responder, Result};
// use diesel::prelude::*;
// use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct MyObj {
    data: String,
}
//read
#[get("/read")]
pub async fn index() -> Result<impl Responder> {
    let obj = MyObj {
        data: "Hello in Rust".to_string(),
    };
    Ok(web::Json(obj))
}
#[derive(Deserialize)]

//post
struct Info {
    first_name: String,
    last_name: String,
    email: String,
}

#[post("/write_post")]
pub async fn write_data(info: web::Json<Info>) -> Result<impl Responder> {
    let connection = establish_connection();
    let first_name = &info.first_name;

    let last_name = &info.last_name;
    let email = &info.email;

    let _post = create_post(&connection, first_name, last_name, email);
    Ok(web::Json("dd"))
}
