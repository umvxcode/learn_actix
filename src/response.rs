use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub result: u16,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            result: 200,
            message: message.to_string(),
            data,
        })
    }

    pub fn error(status: u16, message: &str) -> HttpResponse {
        HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap()).json(ApiResponse {
            result: status,
            message: message.to_string(),
            data: (),  
        })
    }
}
