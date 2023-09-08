use crate::{structs::web::SuccessResponse, util::types::APIResult};
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn req() -> APIResult {
    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "message": "Hello, world!"
        }),
    }))
}
