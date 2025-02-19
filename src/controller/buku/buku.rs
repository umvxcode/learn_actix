use actix_web::{get,post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::model::bukumodel::Entity as Buku;
use sea_orm::EntityTrait;
use crate::db::connect;



#[derive(Serialize, Deserialize)]
struct BukuInput {
    tahun_terbit: i32,
    penulis: String,
    judul: String,
}


#[get("/index")]
async fn index() -> impl Responder {
    let db = connect().await;
    match Buku::find().all(&db).await{
        Ok(buku) => HttpResponse::Ok().json(buku),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[get("/detail/{id}")]
async fn detail(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[derive(Deserialize)]
struct  PostedUpdate{
    username:String
}

#[derive(Serialize)]
struct  UpdateResponse<T>{
    result:u16,
    message:String,
    data:T,
}

#[post("/update")]
async fn update(req: web::Json<PostedUpdate>) -> impl Responder {
    let namad;

    if req.username=="dohawuraijua@gmail.com" {
        namad = "Andre".to_string();
    }else{
        namad ="Bukan andre".to_string();
    }
    let response = UpdateResponse{
        result:200,
        message: format!("username {}",req.username),
        data:namad
    };

    HttpResponse::Ok().json(response)
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(index).service(detail).service(update);
}