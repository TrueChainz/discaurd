use actix::{Actor, Context, Handler, Message};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::{db, helper::hash_verify, tables::generate_token};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: String,
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

        let token_claims = UserClaims {
            id: body.id.to_string(),
            username: body.username,
        };
        let user_session = generate_token(token_claims).unwrap();

        return Ok(user_session);
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

        let body = match user.new(msg.username) {
            Some(data) => data,
            None => return Err("Username does not exist".to_string()),
        };

        let valid_password = hash_verify(msg.password.as_str(), body.password.as_str());

        if valid_password == false {
            return Err("Username and password do not match".to_string());
        }

        let token_claims = UserClaims {
            id: body.id.to_string(),
            username: body.username.to_string(),
        };

        let user_session = generate_token(token_claims).unwrap();

        return Ok(user_session);
    }
}
