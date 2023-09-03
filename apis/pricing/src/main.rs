pub mod db;
pub mod models;
pub mod routes;
pub mod validator;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{web::Data, App, HttpServer};
use routes::{
    external_data_transfer, inter_region_data_transfer, on_demand, spot, spot_forecast, storage,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Per-day governor configuration for 500 requests per day with bursts of 10
    let per_day_conf = GovernorConfigBuilder::default()
        .per_millisecond(288000)
        .burst_size(50)
        .finish()
        .unwrap();

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    // HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Governor::new(&per_day_conf))
            .app_data(Data::new(pool.clone()))
            .service(on_demand)
            .service(spot)
            .service(spot_forecast)
            .service(storage)
            .service(external_data_transfer)
            .service(inter_region_data_transfer)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
