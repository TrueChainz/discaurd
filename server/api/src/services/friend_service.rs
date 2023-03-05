use anyhow::Result;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    db,
    models::{
        friend_model::{FriendQuery, Relation},
        user_model::UserQuery,
    },
};

#[derive(Error, Debug)]
pub enum FriendServiceError {
    #[error("Cannot send request")]
    BadRequest,
    #[error("No status found")]
    NoStatus,
    #[error("User not found")]
    NotFound,
    #[error("Unexpected error! Please try again later")]
    Unexpected,
    #[error("User has blocked you!")]
    Blocked,
    #[error("Database is down")]
    DatabaseError(DbErr),
}

pub async fn add_friend(
    source_username: String,
    target_username: String,
) -> Result<(), FriendServiceError> {
    let db = db::create_client()
        .await
        .map_err(FriendServiceError::DatabaseError)?;
    let user_query = UserQuery { db: db.clone() };
    if &source_username == &target_username {
        return Err(FriendServiceError::BadRequest);
    }

    let source_user = user_query
        .get_by_username(source_username)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::Unexpected)?;
    let target_user = user_query
        .get_by_username(target_username)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::NotFound)?;

    let friend_query = FriendQuery { db: db.clone() };
    let response = friend_query
        .send_request(&Relation {
            user_id: source_user.id,
            target_id: target_user.id,
        })
        .await?;

    return Ok(response);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FriendData {
    pub id: String,
    pub username: String,
}

pub async fn show_pending(username: String) -> Result<Vec<FriendData>, FriendServiceError> {
    let db = db::create_client().await.unwrap();
    let user_query = UserQuery { db: db.clone() };
    let friend_query = FriendQuery { db: db.clone() };

    let user = user_query
        .get_by_id(username)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::Unexpected)?;

    let pending_requests = friend_query
        .get_pending_requests(user.id.clone())
        .await
        .map_err(FriendServiceError::DatabaseError)?;

    let mut pending_requests_info: Vec<FriendData> = vec![];

    for request in pending_requests {
        let pending_friend = user_query
            .get_by_id(request.user_id)
            .await
            .map_err(FriendServiceError::DatabaseError)?;

        if pending_friend.is_some() {
            let full_pending_friend_info = pending_friend.unwrap();
            let pending_friend_info = FriendData {
                id: full_pending_friend_info.id,
                username: full_pending_friend_info.username,
            };
            pending_requests_info.push(pending_friend_info)
        } else {
            // Possibly remove the user from your friend in the future
            println!(
                "One of your pending friend doesn't exist anymore: {:#?}",
                pending_friend
            );
        }
    }

    return Ok(pending_requests_info);
}

pub async fn accept_request(
    source_username: String,
    target_username: String,
) -> Result<FriendData, FriendServiceError> {
    let db = db::create_client().await.unwrap();
    let user_query = UserQuery { db: db.clone() };
    let friend_query = FriendQuery { db: db.clone() };

    let user = user_query
        .get_by_username(source_username)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::Unexpected)?;

    let target_user = user_query
        .get_by_username(target_username)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::Unexpected)?;

    let accept_request = friend_query
        .accept_request(
            &Relation {
                user_id: user.id.clone(),
                target_id: target_user.id,
            },
            user.id,
        )
        .await?;
    let accepted_model = user_query
        .get_by_id(accept_request.user_id)
        .await
        .map_err(FriendServiceError::DatabaseError)?
        .ok_or(FriendServiceError::NotFound)?;

    let accepted_friend = FriendData {
        id: accepted_model.id,
        username: accepted_model.username,
    };

    return Ok(accepted_friend);
}
