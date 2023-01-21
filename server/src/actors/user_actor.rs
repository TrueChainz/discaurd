use actix::{Actor, Context, Handler, Message};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::{db, helper::hash_verify};
use crate::{
    helper::hash_string,
    tables::{User, UserBody, UserSession},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActor;

impl Actor for UserActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<UserSession, String>")]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "Result<UserSession, String>")]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "Result<UserSession, String>")]
pub struct ValidateRefresh {
    pub refresh_token: String,
    pub username: String,
}

impl Handler<Register> for UserActor {
    type Result = Result<UserSession, String>;
    fn handle(&mut self, msg: Register, _ctx: &mut Context<Self>) -> Self::Result {
        let sql_conn = db::sql_connect();
        let mut user = User {
            body: None,
            sql_conn,
        };
        let body = UserBody {
            id: Uuid::new_v4().to_string(),
            username: msg.username,
            email: msg.email,
            password: hash_string(msg.password.as_str()),
        };

        let does_exist = user.check_exist(&body);
        if does_exist == true {
            return Err("Username/email is already used".to_string());
        }

        let insert_response = user.add(&body);
        if insert_response == false {
            return Err("Failed to add user".to_string());
        }

        match user.create_session() {
            Ok(session) => return Ok(session),
            Err(_err) => return Err("Unexpected error occured".to_string()),
        };
    }
}

impl Handler<Login> for UserActor {
    type Result = Result<UserSession, String>;
    fn handle(&mut self, msg: Login, _ctx: &mut Context<Self>) -> Self::Result {
        let sql_conn = db::sql_connect();
        let mut user = User {
            body: None,
            sql_conn,
        };

        let body = match user.get(msg.username) {
            Some(data) => data,
            None => return Err("Username does not exist".to_string()),
        };

        let valid_password = hash_verify(msg.password.as_str(), body.password.as_str());

        if valid_password == false {
            return Err("Username and password do not match".to_string());
        }

        match user.create_session() {
            Ok(session) => return Ok(session),
            Err(_err) => return Err("Unexpected error occured".to_string()),
        }
    }
}

impl Handler<ValidateRefresh> for UserActor {
    type Result = Result<UserSession, String>;
    fn handle(&mut self, msg: ValidateRefresh, _ctx: &mut Context<Self>) -> Self::Result {
        let user = User::get_user_by_username(msg.username);
        if user.body.is_none() {
            return Err("User does not exists".to_string());
        }
        let valid_token = user.validate_session_token(msg.refresh_token);
        if valid_token == false {
            return Err("Token does not exist in session".to_string());
        }
        match user.create_session() {
            Ok(session) => return Ok(session),
            Err(_err) => return Err("Unexpected error occured".to_string()),
        }
    }
}
