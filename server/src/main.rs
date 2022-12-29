use actix_web::{App, HttpServer};
use api::api_config;
use dotenv::dotenv;

mod actors;
mod api;
mod db;
mod helper;
mod tables;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");
    HttpServer::new(move || App::new().configure(api_config))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
