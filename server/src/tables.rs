use chrono::Utc;
use mysql::prelude::*;
use mysql::{params, PooledConn};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{
    actors::user_actor::UserClaims,
    db,
    helper::{generate_token, validate_token, TokenType},
};

#[derive(Debug)]
pub struct UserBody {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl FromRow for UserBody {
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        UserBody {
            id: row.get("id").unwrap(),
            username: row.get("username").unwrap(),
            email: row.get("email").unwrap(),
            password: row.get("password").unwrap(),
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        Ok(UserBody {
            id: row.get("id").unwrap(),
            username: row.get("username").unwrap(),
            email: row.get("email").unwrap(),
            password: row.get("password").unwrap(),
        })
    }
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
    pub fn create(username: String) -> Self {
        let mut conn = db::sql_connect();
        let result_body = conn.exec_first::<UserBody, _, _>(
            "SELECT * FROM users WHERE username = :username",
            params! {"username" => &username},
        );
        if result_body.is_err() {
            return User {
                body: None,
                sql_conn: conn,
            };
        }
        let body = result_body.unwrap();

        return User {
            body,
            sql_conn: conn,
        };
    }
    pub fn get(&mut self, username: String) -> Option<&UserBody> {
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

    pub fn validate_session_token(&self, refresh_token: String) -> bool {
        let valid_token = validate_token(refresh_token, TokenType::AccessToken);
        if valid_token == false {
            return false;
        }
        let mut redis_conn = db::redis_connect();
        let user_body = self.body.as_ref().unwrap();
        let value: Option<String> = redis_conn.get(user_body.id.as_str()).unwrap();

        if value.is_none() {
            return false;
        }

        return true;
    }

    pub fn create_session(self) -> Result<UserSession, String> {
        let mut redis_conn = db::redis_connect();
        if self.body.is_none() {
            return Err("User has does not exist".to_string());
        }

        let body = self.body.unwrap();
        let mut token_claims = UserClaims {
            id: body.id,
            username: body.username,
            exp: Utc::now().timestamp() as usize,
        };
        let access_token = generate_token(&mut token_claims, TokenType::AccessToken);
        let refresh_token = generate_token(&mut token_claims, TokenType::RefreshToken);

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
