use actix_web::{
    delete,
    web::{self},
    HttpResponse,
};
use tokio::sync::Mutex;

use crate::{util::types::APIResult, ServerState};

#[delete("/users/{id}")]
pub async fn req(data: web::Data<Mutex<ServerState>>, id: web::Path<String>) -> APIResult {
    let data = data.lock().await;
    let postgres = &mut data.postgres.clone();

    postgres.delete_user_by_id(id.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}
