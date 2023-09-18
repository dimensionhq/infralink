pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    guard,
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use schema::Query;

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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Governor::new(&per_day_conf))
            .app_data(Data::new(pool.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .app_data(Data::new(schema.clone()))
                    .to(routes::graphql),
            )
            .service(
                web::resource("/graphiql")
                    .guard(guard::Get())
                    .to(routes::graphiql),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
