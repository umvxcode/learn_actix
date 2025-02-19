use actix_web::{ web::{self}, App, HttpServer};

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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