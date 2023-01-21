use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use api::api_config;
use dotenv::dotenv;

mod actors;
mod api;
mod db;
mod helper;
mod models;
mod prisma;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");
    let client = db::create_client().await;
    if client.is_err() {
        panic!("Failed to connect to prisma client");
    }
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1420")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .configure(api_config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
