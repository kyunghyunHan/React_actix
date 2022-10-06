use crate::db::{connection::establish_connection, connection::DbPool, user::create_post};
use actix_web::{get, Responder, Result};
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MyObj {
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
//post
pub struct Info {
    first_name: String,
    last_name: String,
    email: String,
}
#[post("/tweets")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Tweet#new")
}
#[post("/write_post")]
pub async fn write_data(info: web::Json<Info>) -> impl Responder {
    let connection = establish_connection();
    let first_name = &info.first_name;
    let last_name = &info.last_name;
    let email = &info.email;
    // let obj = Info {
    //     first_name: "Hello in Rust".to_string(),
    //     last_name: "Hello in Rust".to_string(),
    //     email: "Hello in Rust".to_string(),
    // };
    let _post = create_post(&connection, first_name, last_name, email);
    HttpResponse::Ok().json(&info.email)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[post("/tweets")]
pub async fn tweet_create() -> impl Responder {
    let connection = establish_connection();
    let obj = Info {
        first_name: "Hello in Rust".to_string(),
        last_name: "Hello in Rust".to_string(),
        email: "Hello in Rust".to_string(),
    };
    let _post = create_post(&connection, &obj.first_name, &obj.last_name, &obj.email);
    HttpResponse::Ok().json(obj)
}
