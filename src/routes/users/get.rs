use crate::{util::types::APIResult, ServerState};
use actix_web::{get, web, HttpResponse};
use tokio::sync::Mutex;

#[get("/users/{id}")]
pub async fn req(data: web::Data<Mutex<ServerState>>, id: web::Path<String>) -> APIResult {
    let data = data.lock().await;
    let postgres = &mut data.postgres.clone();

    let user_data = postgres.get_user_by_id(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(user_data))
}
