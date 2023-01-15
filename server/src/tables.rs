use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{
    db,
    helper::TokenType,
    prisma::{users, PrismaClient},
};

#[derive(Debug)]
pub struct UserBody {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct User {
    pub body: Option<UserBody>,
    pub client: PrismaClient,
}

impl User {
    pub async fn create(username: String) -> Self {
        let client = db::create_client().await.unwrap();
        // let result_body = conn.exec_first::<UserBody, _, _>(
        //     "SELECT * FROM users WHERE username = :username",
        //     params! {"username" => &username},
        // );
        // if result_body.is_err() {
        //     return User {
        //         body: None,
        //         client: conn,
        //     };
        // }
        // let body = result_body.unwrap();

        return User { body: None, client };
    }
    pub async fn get(&mut self, username: String) -> Option<&UserBody> {
        match self
            .client
            .users()
            .find_unique(users::username::equals(username))
            .exec()
            .await
            .unwrap()
        {
            Some(data) => {
                self.body = Some(UserBody {
                    id: data.id,
                    username: data.username,
                    email: data.email,
                    password: data.password,
                });
                return self.body.as_ref();
            }
            _ => return None,
        };
    }
    pub async fn check_exist(&mut self, body: &UserBody) -> bool {
        match self
            .client
            .users()
            .find_unique(users::username::equals(body.username.to_owned()))
            .exec()
            .await
            .unwrap()
        {
            Some(data) => {
                print!("{:?}", data);
                return true;
            }
            _ => return false,
        };
    }
    pub async fn add(&mut self, data: &UserBody) -> bool {
        match self
            .client
            .users()
            .create(
                uuid::Uuid::new_v4().to_string(),
                data.username.as_str().to_string(),
                data.email.as_str().to_string(),
                data.password.as_str().to_string(),
                vec![],
            )
            .exec()
            .await
        {
            Ok(data) => return true,
            _ => return false,
        }
    }
}
