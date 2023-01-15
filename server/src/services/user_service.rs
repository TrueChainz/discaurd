use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db,
    helper::{hash_string, hash_verify},
    tables::{User, UserBody},
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

pub async fn register_user(data: RegisterUser) -> Result<UserInfo, String> {
    let client = db::create_client().await.unwrap();
    let mut user = User { body: None, client };
    let body = UserBody {
        id: Uuid::new_v4().to_string(),
        username: data.username,
        email: data.email,
        password: hash_string(data.password.as_str()),
    };

    let does_exist = user.check_exist(&body).await;
    if does_exist == true {
        return Err("Username/email is already used".to_string());
    }

    let insert_response = user.add(&body).await;
    if insert_response == false {
        return Err("Failed to add user".to_string());
    }

    return Ok(UserInfo {
        id: body.id,
        username: body.username,
    });
}

pub async fn login_user(data: LoginUser) -> Result<UserInfo, String> {
    let client = db::create_client().await.unwrap();

    let mut user = User { body: None, client };

    let body = match user.get(data.username).await {
        Some(data) => data,
        None => return Err("Username does not exist".to_string()),
    };

    let valid_password = hash_verify(data.password.as_str(), body.password.as_str());

    if valid_password == false {
        return Err("Username and password do not match".to_string());
    }

    return Ok(UserInfo {
        id: String::from(body.id.as_str()),
        username: String::from(body.username.as_str()),
    });
}
