use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::friend_service::{add_friend, show_pending, AddFriendPayload, FriendData};

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
        Err(_err) => {
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

#[derive(Deserialize, Serialize, Debug)]
struct ShowPendingRequest {
    username: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShowPendingResponse {
    success: bool,
    error_message: String,
    friends: Vec<FriendData>,
}

#[get("/pending")]
async fn pending(data: web::Query<ShowPendingRequest>) -> impl Responder {
    let show_pending = show_pending(data.0.username).await;
    println!("PENDING FRIENDS: {:#?}", show_pending);

    match show_pending {
        Ok(pending_friends) => {
            let response = ShowPendingResponse {
                success: true,
                error_message: "".to_string(),
                friends: pending_friends,
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(_err) => {
            let response = ShowPendingResponse {
                success: false,
                error_message: "Hm, didn't works. You might not exist!".to_string(),
                friends: vec![],
            };
            return HttpResponse::NotFound().json(json!(response));
        }
    }
}

#[post("/delete")]
async fn delete(_data: web::Json<AddFriendRequest>) -> impl Responder {
    return HttpResponse::BadRequest().json(json!({}));
}

#[post("/block")]
async fn block(_data: web::Json<AddFriendRequest>) -> impl Responder {
    return HttpResponse::BadRequest().json(json!({}));
}

pub fn friend_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/friend").service(add).service(pending));
}
