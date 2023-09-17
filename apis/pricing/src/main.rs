pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod validator;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Per-day governor configuration for 335 requests per day with bursts of 10
    let per_day_conf = GovernorConfigBuilder::default()
        .per_millisecond(288000)
        .burst_size(35)
        .finish()
        .unwrap();

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    // HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Governor::new(&per_day_conf))
            .app_data(Data::new(pool.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
