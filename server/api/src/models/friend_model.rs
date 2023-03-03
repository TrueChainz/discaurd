use prisma_client_rust::{and, or};

use crate::prisma::{
    friends::{self, Data},
    FriendStatus, PrismaClient,
};

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
pub struct Friend {
    pub client: PrismaClient,
}

impl Friend {
    pub async fn add_friend(&self, data: &Relation) -> bool {
        match self
            .client
            .friends()
            .create(
                uuid::Uuid::new_v4().to_string(),
                (*data.user_id).to_string(),
                (*data.target_id).to_string(),
                vec![friends::status::set(FriendStatus::Pending)],
            )
            .exec()
            .await
        {
            Ok(_data) => return true,
            _ => return false,
        }
    }
    pub async fn update_status(&self, friendship_id: String, status: FriendStatus) -> Data {
        return self
            .client
            .friends()
            .update(
                friends::id::equals(friendship_id),
                vec![friends::status::set(status)],
            )
            .exec()
            .await
            .unwrap();
    }
    pub async fn get_status(&self, data: &Relation) -> Option<Data> {
        return self
            .client
            .friends()
            .find_first(vec![or![
                and!(
                    friends::user_id::equals((*data.user_id).to_string()),
                    friends::friend_id::equals((*data.target_id).to_string()),
                ),
                and!(
                    friends::user_id::equals((*data.target_id).to_string()),
                    friends::friend_id::equals((*data.user_id).to_string())
                ),
            ]])
            .exec()
            .await
            .unwrap();
    }

    pub async fn get_pending_requests(&self, user_id: String) -> Vec<Data> {
        let pending_requests = self
            .client
            .friends()
            .find_many(vec![
                and!(friends::friend_id::equals(user_id)),
                and!(friends::status::equals(FriendStatus::Pending)),
            ])
            .exec()
            .await
            .unwrap();
        return pending_requests;
    }

    pub async fn send_request(&self, data: &Relation) -> Result<(), String> {
        let friendship = self.get_status(data).await;
        if friendship.is_some() {
            let status = friendship.unwrap().status;
            match status {
                FriendStatus::Accepted => return Err("Already your friend".to_string()),
                FriendStatus::Pending => return Ok(()),
                FriendStatus::Blocked => return Err("User has blocked you!".to_string()),
            };
        }

        let response = self.add_friend(&data).await;
        if response == false {
            return Err("Unexpected Error has occured, please try again later!".to_string());
        }
        return Ok(());
    }
    pub async fn accept_request(&self, data: &Relation, user_id: String) -> Result<Data, &str> {
        if let Some(friendship) = self.get_status(data).await {
            match friendship.status {
                FriendStatus::Accepted => return Err("Already your friend"),
                FriendStatus::Pending => {
                    if friendship.friend_id == user_id {
                        let friendship_data = self
                            .update_status(friendship.id, FriendStatus::Accepted)
                            .await;
                        return Ok(friendship_data);
                    }
                    return Err("You can't accept for yourself lol");
                }
                FriendStatus::Blocked => return Err("User has blocked you!"),
            }
        }

        return Err("No status somehow!");
    }
}
