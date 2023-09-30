pub mod api;
pub mod cost;
pub mod db;
mod git;
pub mod github;
pub mod models;
pub mod webhook;

use actix_web::{web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Connect to the database
    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(webhook::webhook)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}
