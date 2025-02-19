use actix_web::{get,post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};



#[get("/index")]
async fn index(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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