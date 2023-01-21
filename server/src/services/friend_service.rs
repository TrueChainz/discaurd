pub struct AddFriendPayload {
    pub source_username: String,
    pub target_username: String,
}

use crate::{
    db,
    models::{
        friend_model::{Friend, Relation},
        user_model::User,
    },
};

pub async fn add_friend(data: AddFriendPayload) -> Result<(), String> {
    let client = db::create_client().await.unwrap();
    if &data.source_username == &data.target_username {
        return Err("Cannot add yourself.".to_string());
    }
    let user = User::get_user_by_username((&data).source_username.to_string()).await;
    let target_user = User::get_user_by_username((&data).target_username.to_string()).await;

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
