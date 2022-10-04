use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[derive(Serialize)]
struct MyObj {
    data: String,
}
#[get("/ed")]
async fn index() -> Result<impl Responder> {
    let obj = MyObj {
        data: "Hello in Rust".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

        App::new()
            .wrap(cors) // 해결!
            .service(index)
            .service(echo)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
