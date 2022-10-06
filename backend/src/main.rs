extern crate actix_web;
#[macro_use]
extern crate diesel;
use actix_cors::Cors;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(db::connection::init_pool());
    HttpServer::new(move || {
        let cors = Cors::default()
            .max_age(3600)
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "GET"]);
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(routes::user::echo)
            .service(routes::user::tweet_create)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
