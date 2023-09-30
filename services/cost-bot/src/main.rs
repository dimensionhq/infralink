pub mod api;
pub mod cost;
mod git;
pub mod models;
pub mod webhook;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| App::new().service(webhook::webhook))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
