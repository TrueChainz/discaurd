use ::entity::friends::{ActiveModel, Column, Model, Status};
use ::entity::prelude::Friends;
use chrono::Local;
use sea_orm::*;
use uuid::Uuid;

use crate::services::friend_service::FriendServiceError;

#[derive(Debug)]
pub struct Relation {
    pub user_id: String,
    pub target_id: String,
}

#[derive(Debug)]
pub struct UserBody {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct FriendQuery {
    pub db: DbConn,
}

impl FriendQuery {
    pub async fn add_friend(&self, data: &Relation) -> Result<Model, FriendServiceError> {
        let active_model = ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4().to_string()),
            user_id: ActiveValue::Set(data.user_id.clone()),
            friend_id: ActiveValue::Set(data.target_id.clone()),
            status: ActiveValue::Set(Status::Pending),
            submitted_at: ActiveValue::Set(Local::now().naive_utc()),
        };

        let friend_model = active_model
            .insert(&self.db)
            .await
            .map_err(FriendServiceError::DatabaseError)?;

        return Ok(friend_model);
    }
    pub async fn update_status(
        &self,
        friendship_model: Model,
        status: Status,
    ) -> Result<Model, DbErr> {
        let mut active_model: ActiveModel = friendship_model.into();
        active_model.status = ActiveValue::Set(status);
        let updated_model = active_model.update(&self.db).await?;

        return Ok(updated_model);
    }
    pub async fn get_status(&self, data: &Relation) -> Result<Option<Model>, DbErr> {
        let body = Friends::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(Column::UserId.eq(data.user_id.clone()))
                            .add(Column::FriendId.eq(data.target_id.clone())),
                    )
                    .add(
                        Condition::all()
                            .add(Column::UserId.eq(data.target_id.clone()))
                            .add(Column::FriendId.eq(data.user_id.clone())),
                    ),
            )
            .one(&self.db)
            .await?;
        return Ok(body);
    }

    pub async fn get_pending_requests(&self, user_id: String) -> Result<Vec<Model>, DbErr> {
        let pending_requests = Friends::find()
            .filter(
                Condition::all()
                    .add(Column::FriendId.eq(user_id))
                    .add(Column::Status.eq(Status::Pending)),
            )
            .all(&self.db)
            .await?;

        return Ok(pending_requests);
    }

    pub async fn send_request(&self, data: &Relation) -> Result<(), FriendServiceError> {
        let friendship = self
            .get_status(data)
            .await
            .map_err(FriendServiceError::DatabaseError)?;
        if friendship.is_some() {
            let status = friendship.unwrap().status;
            match status {
                Status::Accepted => return Err(FriendServiceError::BadRequest),
                Status::Pending => return Ok(()),
                Status::Blocked => return Err(FriendServiceError::Blocked),
            };
        }

        self.add_friend(&data).await?;

        return Ok(());
    }
    pub async fn accept_request(
        &self,
        data: &Relation,
        user_id: String,
    ) -> Result<Model, FriendServiceError> {
        if let Some(friendship) = self
            .get_status(data)
            .await
            .map_err(FriendServiceError::DatabaseError)?
        {
            match friendship.status {
                Status::Accepted => return Err(FriendServiceError::BadRequest),
                Status::Pending => {
                    if friendship.friend_id == user_id {
                        let updated_model = self
                            .update_status(friendship, Status::Accepted)
                            .await
                            .map_err(FriendServiceError::DatabaseError)?;
                        return Ok(updated_model);
                    }
                    return Err(FriendServiceError::BadRequest);
                }
                Status::Blocked => return Err(FriendServiceError::Blocked),
            }
        }

        return Err(FriendServiceError::NoStatus);
    }
}
