use serde::{Deserialize, Serialize};

use crate::{
    db,
    models::{
        friend_model::{Friend, Relation},
        user_model::User,
    },
};

pub async fn add_friend(source_username: String, target_username: String) -> Result<(), String> {
    let client = db::create_client().await.unwrap();
    if &source_username == &target_username {
        return Err("Cannot add yourself.".to_string());
    }
    let user = User::get_user_by_username(source_username).await;
    let target_user = User::get_user_by_username(target_username).await;

    if user.body.is_none() {
        return Err("Unexpected error! Please try again later.".to_string());
    }

    if target_user.body.is_none() {
        return Err("User does not exist!".to_string());
    }

    let friend = Friend { client };
    let response = friend
        .send_request(&Relation {
            user_id: user.body.unwrap().id,
            target_id: target_user.body.unwrap().id,
        })
        .await;

    if response.is_err() {
        return Err(response.unwrap_err());
    }
    return Ok(());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendData {
    pub id: String,
    pub username: String,
}

pub async fn show_pending(username: String) -> Result<Vec<FriendData>, String> {
    let client = db::create_client().await.unwrap();

    let user = &User::get_user_by_username(username).await;

    if user.body.is_none() {
        return Err("Unexpected error! Please try again later.".to_string());
    }

    let friend = Friend { client };

    let pending_requests = friend
        .get_pending_requests(user.body.as_ref().unwrap().id.clone())
        .await;

    let mut pending_requests_info: Vec<FriendData> = vec![];

    for request in &pending_requests {
        let pending_friend = User::get_user_by_id(request.user_id.clone()).await;

        if pending_friend.body.is_some() {
            let full_pending_friend_info = pending_friend.body.unwrap();
            let pending_friend_info = FriendData {
                id: full_pending_friend_info.id,
                username: full_pending_friend_info.username,
            };
            pending_requests_info.push(pending_friend_info)
        }
    }

    return Ok(pending_requests_info);
}

pub async fn accept_request(
    source_username: String,
    target_username: String,
) -> Result<Vec<FriendData>, &'static str> {
    let client = db::create_client().await.unwrap();

    let user = User::get_user_by_username(source_username).await;
    let target_user = User::get_user_by_username(target_username).await;

    if user.body.is_none() {
        return Err("Unexpected error! Please try again later.");
    }

    if target_user.body.is_none() {
        return Err("Unexpected error! Please try again later.");
    }

    let friend = Friend { client };

    let accept_request = friend
        .accept_request(
            &Relation {
                user_id: user.body.as_ref().unwrap().id.to_string(),
                target_id: target_user.body.unwrap().id,
            },
            user.body.as_ref().unwrap().id.to_string(),
        )
        .await;

    if accept_request.is_err() {
        return Err("Unexpected error! Please try again later.");
    }

    let pending_requests_info: Vec<FriendData> = vec![];

    return Ok(pending_requests_info);
}
