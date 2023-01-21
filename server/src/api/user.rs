use actix::Actor;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::user_service::{login_user, register_user, LoginUser, RegisterUser, UserInfo};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RegisterResponse {
    success: bool,
    error_message: String,
    user: Option<UserInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    success: bool,
    error_message: String,
    user: Option<UserInfo>,
}

#[post("/register")]
async fn register(data: web::Json<RegisterRequest>) -> impl Responder {
    let register_result = register_user(RegisterUser {
        email: data.0.email,
        username: data.0.username,
        password: data.0.password,
    })
    .await;
    println!("{:?}", register_result);

    match register_result {
        Ok(user_info) => {
            let response = RegisterResponse {
                success: true,
                error_message: "".to_string(),
                user: Some(user_info),
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(err) => {
            let response = RegisterResponse {
                success: false,
                error_message: err,
                user: None,
            };
            return HttpResponse::BadRequest().json(json!(response));
        }
    }
}

#[post("/login")]
async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    let login_result = login_user(LoginUser {
        username: data.0.username,
        password: data.0.password,
    })
    .await;
    println!("{:?}", login_result);

    match login_result {
        Ok(user_info) => {
            let response = LoginResponse {
                success: true,
                error_message: "".to_string(),
                user: Some(user_info),
            };
            return HttpResponse::Created().json(json!(response));
        }
        Err(err) => {
            let response = LoginResponse {
                success: false,
                error_message: err,
                user: None,
            };
            return HttpResponse::NotFound().json(json!(response));
        }
    }
}

#[get("/authenticate")]
async fn authenticate(request: HttpRequest) -> impl Responder {
    return HttpResponse::Created().json(json!({ "response": "authenticate" }));

    // let headers = request.headers();
    // let authorization = headers.get("authorization");

    // let value = match authorization {
    //     Some(value) => value,
    //     None => return HttpResponse::Unauthorized(),
    // };

    // let token_list = value.to_str().unwrap().split(" ").collect::<Vec<&str>>();
    // let token_option = token_list.get(1);

    // let token = match token_option {
    //     Some(token) => token.to_string(),
    //     None => return HttpResponse::Unauthorized(),
    // };
    // match validate_token(token, TokenType::AccessToken) {
    //     true => return HttpResponse::Ok(),
    //     false => return HttpResponse::Unauthorized(),
    // }
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(register)
            .service(login)
            .service(authenticate),
    );
}
