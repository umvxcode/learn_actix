use actix_web::{ web::{self}, App, HttpServer};
use dotenvy::dotenv;
use env_logger;

mod db;
mod router;
mod model;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .configure(router::config)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}