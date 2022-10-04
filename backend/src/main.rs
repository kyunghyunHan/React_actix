extern crate actix_web;

#[macro_use]
extern crate r2d2;
#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::mysql::*;

use r2d2::PooledConnection;

use r2d2_diesel::ConnectionManager;
mod routes;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct Conn(pub PooledConnection<ConnectionManager<MysqlConnection>>);
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        dotenv::dotenv().ok();

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
