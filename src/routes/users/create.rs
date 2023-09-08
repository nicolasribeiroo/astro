use crate::{
    connectivity::postgres::User,
    structs::web::{ErrorResponse, ErrorStruct, SuccessResponse},
    util::types::APIResult,
    ServerState,
};
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
    if payload.username.len() > 32 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: ErrorStruct {
                code: "username_too_long".to_string(),
                message: "Username is too long".to_string(),
            },
        }));
    }

    if payload.username.len() < 3 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: ErrorStruct {
                code: "username_too_short".to_string(),
                message: "Username is too short".to_string(),
            },
        }));
    }

    if !validator::validate_email(&payload.email) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: ErrorStruct {
                code: "invalid_email".to_string(),
                message: "Email is invalid".to_string(),
            },
        }));
    }

    let data = data.lock().await;
    let postgres = &mut data.postgres.clone();

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: payload.username.clone(),
        password: payload.password.clone(),
        email: payload.email.clone(),
    };

    let user_data = postgres.insert_new_user(user).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: serde_json::json!(user_data),
    }))
}
