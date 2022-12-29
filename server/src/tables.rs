use jsonwebtoken::{encode, EncodingKey, Header};
use mysql::{params, prelude::*, PooledConn};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{actors::user_actor::UserClaims, db};

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

        return true;
    }
}

pub fn generate_token(token_claims: UserClaims) -> Result<UserSession, String> {
    let mut redis_conn = db::redis_connect();
    let access_token = match encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret("access_token_key".as_ref()),
    ) {
        Ok(token) => token,
        Err(_err) => return Err("Failed generating access".to_string()),
    };
    let refresh_token = match encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret("refresh_token_key".as_ref()),
    ) {
        Ok(token) => token,
        Err(_err) => return Err("Failed generating refresh".to_string()),
    };
    let _: () = redis_conn
        .set(token_claims.id.as_str(), &refresh_token)
        .unwrap();

    let user_session = UserSession {
        access_token,
        refresh_token,
    };

    return Ok(user_session);
}
