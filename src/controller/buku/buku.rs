use actix_web::{delete, get, post, web, HttpResponse, Responder};

use serde::Deserialize;
use serde::Serialize;
use crate::model::bukumodel::Entity as Buku;
use crate::model::bukumodel::ActiveModel as BukuActive;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use crate::db::connect;
use crate::response::ApiResponse;
use crate::helper::tanggal_indo;


#[derive(Deserialize)]
struct  PostedUpdate{
    pub id: i32,
    pub tahun_terbit: i32,
    pub penulis: String,
    pub judul: String,
}


#[derive(Deserialize)]
struct  PostedCreate{
    pub tahun_terbit: i32,
    pub penulis: String,
    pub judul: String,
}


#[derive(Serialize)]
struct BukuResponse {
    id: i32,
    tahun_terbit: i32,
    penulis: String,
    judul: String,
    keterangan: String,
    updated_at: String
}

#[get("/index")]
async fn index() -> impl Responder {
    let db = connect().await;

    match Buku::find().all(&db).await {
        Ok(buku_list) => {
            let response : Vec<BukuResponse> = buku_list.into_iter()
            .map(|buku|BukuResponse {
                id:buku.id,
                tahun_terbit:buku.tahun_terbit,
                penulis:buku.penulis,
                judul:buku.judul,
                keterangan: if buku.tahun_terbit < 2024 {
                    "buku lama".to_string()
                }else{
                    "buku baru".to_string()
                },
                updated_at: tanggal_indo(buku.updated_at.to_string(),"full"), 
            }).collect();
            ApiResponse::success("Sukses",response)
        } 
        
        Err(_) => ApiResponse::<()>::error(500, "Internal server error"),
    }
}

#[post("/create")]
async fn create_buku(buku: web::Json<PostedCreate>) -> impl Responder {
    let db = connect().await;
    let new_buku = BukuActive {
        id: Default::default(),
        tahun_terbit: Set(buku.tahun_terbit),
        penulis: Set(buku.penulis.clone()),
        judul: Set(buku.judul.clone()),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    match new_buku.insert(&db).await {
        Ok(saved_buku) => ApiResponse::success("Data berhasil disimpan", saved_buku),
        Err(_) => ApiResponse::<()>::error(500, "Internal server error"),
    }
}


#[get("/byid/{id}")]
async fn detail(id: web::Path<i32>) -> impl Responder {
    let db = connect().await;

    match Buku::find_by_id(id.into_inner()).one(&db).await {
        Ok(Some(buku))=> ApiResponse::success("Sukses", buku),
        Ok(None) =>ApiResponse::<()>::error(404, "Data not found"),
        Err(_)=>ApiResponse::<()>::error(500, "Internal server error"),
    }
}

#[post("/update")]
async fn update(req: web::Json<PostedUpdate>) -> impl Responder {
    let db = connect().await;
    let buku_id = req.id;

    match Buku::find_by_id(buku_id).one(&db).await {
        Ok(Some(existing_buku))=>{
            let mut buku_active: BukuActive=existing_buku.into();
            buku_active.tahun_terbit = Set(req.tahun_terbit);
            buku_active.penulis = Set(req.penulis.clone());
            buku_active.judul=Set(req.judul.clone());
            buku_active.updated_at = Set(chrono::Utc::now().into());

            match buku_active.update(&db).await {
                Ok(updated_buku)=> ApiResponse::success("Data berhasi diperbaharui", updated_buku),
                Err(_) => ApiResponse::<()>::error(500, "Internal Server Error"),
            }
        }
        Ok(None)=>ApiResponse::<()>::error(404, "Data not found"),
        Err(_) => ApiResponse::<()>::error(500, "Internal Server Error"),
    }

}

#[delete("delete/{id}")]
async fn delete_buku(id: web::Path<i32>) -> impl Responder{
    let db = connect().await;
    let buku_id = id.into_inner();

    match Buku::find_by_id(buku_id).one(&db).await {
        Ok(Some(existing_buku))=>{
            let buku_active: BukuActive = existing_buku.into();
            match buku_active.delete(&db).await {
                Ok(_)=>HttpResponse::Ok().body("Delete sukses"),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => ApiResponse::<()>::error(404, "Data not found"),
        Err(_) => ApiResponse::<()>::error(500, "Internal Server Error"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(index).service(detail).service(update).service(create_buku);
}