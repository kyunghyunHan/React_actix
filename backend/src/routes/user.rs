use crate::db::connection;
use crate::db::schema::users;
// use crate::db::user::create_post;
use actix_web::{get, post, web, HttpResponse, Responder, Result};

use serde::Serialize;
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
// #[post("/write_post")]
// async fn write_data(info: web::Json<User>) -> String {
//     let connection = connection::init_pool();
//     let title = &info.title;

//     let body = &info.body;

//     let _post = create_post(&connection, title, &body);
//     format!("hey")
// }
