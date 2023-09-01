pub mod db;
pub mod models;
pub mod routes;
pub mod validator;

use actix_web::{web::Data, App, HttpServer};
use routes::{external_data_transfer, inter_region_data_transfer, on_demand, spot, spot_forecast};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(spot_forecast)
            .service(external_data_transfer)
            .service(inter_region_data_transfer)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
