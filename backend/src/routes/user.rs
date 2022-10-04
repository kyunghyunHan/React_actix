use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Serialize;
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[derive(Serialize)]
struct MyObj {
    data: String,
}
#[get("/read")]
pub async fn index() -> Result<impl Responder> {
    let obj = MyObj {
        data: "Hello in Rust".to_string(),
    };
    Ok(web::Json(obj))
}
