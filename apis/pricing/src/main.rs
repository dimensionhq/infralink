pub mod constants;
pub mod db;
pub mod models;
pub mod routes;
pub mod validator;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use routes::{on_demand, spot};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env
    dotenv().ok();

    // Create env_logger with log level set to INFO
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Database URL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create connection
    let pool = db::create_pool(&database_url).await;

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(on_demand)
            .service(spot)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
