use anyhow::Result;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::create_client,
    helper::{hash_string, hash_verify},
    models::user_model::{UserBody, UserQuery},
};

pub struct LoginUser {
    pub username: String,
    pub password: String,
}

pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Username/email is already used")]
    AlreadyExists,
    #[error("User not found")]
    NotFound,
    #[error("Username and password do not match")]
    InvalidCredentials,
    #[error("Database is down")]
    DatabaseError(DbErr),
}

pub async fn register_user(data: RegisterUser) -> Result<UserInfo, UserServiceError> {
    let db = create_client()
        .await
        .map_err(UserServiceError::DatabaseError)?;

    let user = UserQuery { db };

    let body = UserBody {
        id: Uuid::new_v4().to_string(),
        username: data.username,
        email: data.email,
        password: hash_string(data.password.as_str()),
    };

    let does_exist = user
        .does_exist(&body)
        .await
        .map_err(UserServiceError::DatabaseError)?;
    if does_exist == true {
        return Err(UserServiceError::AlreadyExists);
    }

    let insert_response = user
        .register(&body)
        .await
        .map_err(UserServiceError::DatabaseError)?;

    return Ok(UserInfo {
        id: body.id,
        username: body.username,
    });
}

pub async fn login_user(data: LoginUser) -> Result<UserInfo, UserServiceError> {
    let db = create_client()
        .await
        .map_err(UserServiceError::DatabaseError)?;

    let user = UserQuery { db };

    let body = user
        .get_by_username(data.username)
        .await
        .map_err(UserServiceError::DatabaseError)?
        .ok_or(UserServiceError::NotFound)?;

    let valid_password = hash_verify(data.password.as_str(), body.password.as_str());
    if valid_password == false {
        return Err(UserServiceError::InvalidCredentials);
    }

    return Ok(UserInfo {
        id: String::from(body.id.as_str()),
        username: String::from(body.username.as_str()),
    });
}
