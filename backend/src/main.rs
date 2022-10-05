extern crate actix_web;
#[macro_use]
extern crate diesel;
use actix_cors::Cors;

use actix_web::{App, HttpServer};

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

        App::new()
            .app_data(db::connection::init_pool())
            .wrap(cors) // 해결!
            .service(routes::user::write_data)
            .service(routes::user::index)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
