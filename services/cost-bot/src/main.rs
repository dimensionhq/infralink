pub mod api;
pub mod cost;
pub mod models;
pub mod webhook;
mod git;

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
