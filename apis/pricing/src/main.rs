pub mod pricing;
pub mod routes;

// use actix_web::{App, HttpServer};
// use routes::get_pricing_list;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    pricing::get_pricing_for_region("us-east-1").await.unwrap();

    // HttpServer::new(|| App::new().service(get_pricing_list))
    //     .bind("127.0.0.1:8080")?
    //     .run()
    //     .await

    Ok(())
}
