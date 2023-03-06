use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use api::api_config;

pub use sea_orm;

mod api;
mod db;
mod helper;
mod models;
mod services;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    println!("Hello world");
    // let host =: env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");

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

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
