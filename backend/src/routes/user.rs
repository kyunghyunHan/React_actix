use crate::db::{connection::establish_connection, user::create_post};

use actix_web::{get, Responder, Result};
use actix_web::{post, web, Error, HttpResponse};
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
pub struct Info {
    first_name: String,
    last_name: String,
    email: String,
}
// #[post("/tweets")]
// pub async fn create() -> impl Responder {
//   HttpResponse::Ok().body("Tweet#new")
// }
// #[post("/write_post")]
// pub async fn write_data(info: web::Json<Info>) -> Result<impl Responder> {
//     let connection = establish_connection();
//     let first_name = &info.first_name;
//     let last_name = &info.last_name;
//     let email = &info.email;
//     let obj = Info {
//         first_name: "Hello in Rust".to_string(),
//         last_name: "Hello in Rust".to_string(),
//         email: "Hello in Rust".to_string(),
//     };
//     let _post = create_post(&connection, first_name, last_name, email);
//     Ok(web::Json(obj))
// }

// s
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
