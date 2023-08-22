use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PricingRequest {
    regions: Vec<String>,
    cloud_provider: String,
}

#[post("/pricing-list")]
async fn get_pricing_list(data: web::Json<PricingRequest>) -> impl Responder {
    // Call your pricing update function here
    // pricing::update_pricing_list().await;

    HttpResponse::Ok()
}
