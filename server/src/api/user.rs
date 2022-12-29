use crate::{
    actors::user_actor::{Login, Register, UserActor},
    tables::UserSession,
};
use actix::Actor;
use actix_web::{post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RegisterResponse {
    status: u16,
    error_message: String,
    user_session: Option<UserSession>,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    status: u16,
    error_message: String,
    user_session: Option<UserSession>,
}

#[put("/register")]
async fn register(data: web::Json<RegisterRequest>) -> impl Responder {
    let addr = UserActor {}.start();
    let register = addr
        .send(Register {
            username: data.0.username,
            email: data.0.email,
            password: data.0.password,
        })
        .await
        .unwrap();

    match register {
        Ok(session) => {
            let response = RegisterResponse {
                status: 201,
                error_message: "".to_string(),
                user_session: Some(session),
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(err) => {
            let response = RegisterResponse {
                status: 404,
                error_message: err,
                user_session: None,
            };
            return HttpResponse::BadRequest().json(json!(response));
        }
    }
}

#[post("/login")]
async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    let addr = UserActor {}.start();
    let login = addr
        .send(Login {
            username: data.0.username,
            password: data.0.password,
        })
        .await
        .unwrap();
    match login {
        Ok(session) => {
            let response = RegisterResponse {
                status: 201,
                error_message: "".to_string(),
                user_session: Some(session),
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(err) => {
            let response = RegisterResponse {
                status: 201,
                error_message: err,
                user_session: None,
            };
            return HttpResponse::BadRequest().json(json!(response));
        }
    }
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(register).service(login));
}
