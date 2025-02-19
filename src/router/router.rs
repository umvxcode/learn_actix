use actix_web::web;

#[path = "../controller/mod.rs"]

mod buku;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
       web::scope("/buku").configure(buku::buku::buku::init_routes)
    );
}

