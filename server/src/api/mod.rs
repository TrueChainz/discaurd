use self::{friend::friend_config, user::user_config};
use actix_web::web;

mod friend;
mod user;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user_config)
            .configure(friend_config),
    );
}
