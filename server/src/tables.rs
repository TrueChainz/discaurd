use std::ops::Add;

use chrono::{Duration, Utc};
use mysql::{params, prelude::*, PooledConn};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{
    actors::user_actor::UserClaims,
    db,
    helper::{generate_token, TokenType},
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
    pub sql_conn: PooledConn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub access_token: String,
    pub refresh_token: String,
}

impl User {
    pub fn new(&mut self, username: String) -> Option<&UserBody> {
        let mut row = match self
            .sql_conn
            .exec_iter(
                "SELECT * FROM users WHERE username = :username",
                params! {"username" => &username},
            )
            .unwrap()
            .next()
        {
            Some(Ok(row)) => row,
            _ => return None,
        };

        self.body = Some(UserBody {
            id: row.take("id").unwrap(),
            username: row.take("username").unwrap(),
            email: row.take("email").unwrap(),
            password: row.take("password").unwrap(),
        });

        return self.body.as_ref();
    }
    pub fn check_exist(&mut self, body: &UserBody) -> bool {
        match self
            .sql_conn
            .exec_iter(
                "SELECT id, username, email, password FROM users WHERE username = :username OR email = :email ",
                params! {
                    "username" => &body.username,
                    "email" => &body.email
                },
            )
            .unwrap()
            .next()
        {
            Some(Ok(_)) => return true,
            _ => return false,
        }
    }
    pub fn add(&mut self, data: &UserBody) -> bool {
        let response = self.sql_conn.exec_drop(
            r"INSERT INTO users (id, username, email, password) VALUES (:id, :username, :email, :password)",
            params! {
                "id" => &data.id,
                "username" => &data.username,
                "email" => &data.email,
                "password" => &data.password,
            },
        );

        if response.is_err() {
            let err = response.unwrap_err();
            println!("Adding error: {}", err);
            return false;
        };

        self.body = Some(UserBody {
            id: data.id.to_string(),
            username: data.username.to_string(),
            email: data.email.to_string(),
            password: data.password.to_string(),
        });
        return true;
    }
    pub fn create_session(self) -> Result<UserSession, String> {
        let mut redis_conn = db::redis_connect();
        if self.body.is_none() {
            return Err("User has does not exist".to_string());
        }

        let body = self.body.unwrap();
        let token_claims = UserClaims {
            id: body.id,
            username: body.username,
            exp: Utc::now().add(Duration::minutes(30)).timestamp() as usize,
        };
        let access_token = generate_token(&token_claims, TokenType::AccessToken);
        let refresh_token = generate_token(&token_claims, TokenType::RefreshToken);

        redis_conn
            .set::<_, _, ()>(token_claims.id.as_str(), &refresh_token)
            .unwrap();

        let user_session = UserSession {
            access_token,
            refresh_token,
        };

        return Ok(user_session);
    }
}
