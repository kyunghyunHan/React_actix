use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod routes;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

        App::new()
            .wrap(cors) // 해결!
            .service(routes::user::index)
            .service(routes::user::echo)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
