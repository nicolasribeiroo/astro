use crate::util::types::APIResult;
use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DataResponse {
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub success: bool,
    pub data: DataResponse,
}

#[get("/")]
pub async fn req() -> APIResult {
    let response = Response {
        success: true,
        data: DataResponse {
            message: "Hello, World!".to_string(),
        },
    };

    Ok(HttpResponse::Ok().json(response))
}
