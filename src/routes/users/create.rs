use crate::{connectivity::postgres::User, util::types::APIResult, ServerState};
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    password: String,
    email: String,
}

#[post("/users")]
pub async fn req(data: web::Data<Mutex<ServerState>>, payload: web::Json<CreateUser>) -> APIResult {
    let data = data.lock().await;
    let postgres = &mut data.postgres.clone();

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: payload.username.clone(),
        password: payload.password.clone(),
        email: payload.email.clone(),
    };

    let user_data = postgres.insert_new_user(user).await?;

    Ok(HttpResponse::Ok().json(user_data))
}
