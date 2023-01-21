use actix::Actor;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::friend_service::{add_friend, AddFriendPayload};

#[derive(Deserialize, Serialize, Debug)]
struct AddFriendRequest {
    source_username: String,
    target_username: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddFriendResponse {
    success: bool,
    error_message: String,
}

#[post("/add")]
async fn add(data: web::Json<AddFriendRequest>) -> impl Responder {
    let add_friend_result = add_friend(AddFriendPayload {
        source_username: data.0.source_username,
        target_username: data.0.target_username,
    })
    .await;
    println!("{:?}", add_friend_result);
    match add_friend_result {
        Ok(()) => {
            let response = AddFriendResponse {
                success: true,
                error_message: "".to_string(),
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(err) => {
            let response = AddFriendResponse {
                success: false,
                error_message:
                    "Hm, didn't work. Double check that capitalisations and spelling are corect."
                        .to_string(),
            };
            return HttpResponse::NotFound().json(json!(response));
        }
    }
}

#[post("/delete")]
async fn delete(data: web::Json<AddFriendRequest>) -> impl Responder {
    return HttpResponse::BadRequest().json(json!({}));
}

#[post("/block")]
async fn block(data: web::Json<AddFriendRequest>) -> impl Responder {
    return HttpResponse::BadRequest().json(json!({}));
}

pub fn friend_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/friend").service(add).service(delete));
}
