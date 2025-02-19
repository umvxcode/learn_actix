use actix_web::{ web::{self}, App, HttpServer};
use dotenvy::dotenv;

mod db;
mod router;
mod model;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .configure(router::config)
            )

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}