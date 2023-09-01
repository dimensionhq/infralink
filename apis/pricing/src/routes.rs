use super::models::on_demand_request::OnDemandRequest;
use crate::{
    db,
    models::{
        external_data_transfer_request::ExternalDataTransferRequest,
        inter_region_data_transfer_request::InterRegionDataTransferRequest,
        spot_request::SpotRequest,
    },
    validator,
};
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

#[post("/pricing/on-demand")]
pub async fn on_demand(
    pool: web::Data<Pool<Postgres>>,
    req_body: web::Json<OnDemandRequest>,
) -> impl Responder {
    // Validate the request
    if validator::validate_on_demand_request(req_body.clone()) {
        // Fetch the on-demand data
        let result = db::fetch_on_demand_data(&pool, req_body.into_inner()).await;

        match result {
            Ok(data) => {
                let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string());
                HttpResponse::Ok().body(json_data)
            }
            Err(e) => {
                eprintln!("Error fetching data: {:?}", e);
                HttpResponse::InternalServerError().body("Data fetching failed")
            }
        }
    } else {
        HttpResponse::BadRequest().body("Invalid Request Body. Check your parameters.")
    }
}

#[post("/pricing/spot")]
pub async fn spot(
    pool: web::Data<Pool<Postgres>>,
    req_body: web::Json<SpotRequest>,
) -> impl Responder {
    if validator::validate_spot_request(req_body.clone()) {
        let result = db::fetch_spot_data(&pool, req_body.into_inner()).await;

        match result {
            Ok(data) => {
                let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string());
                HttpResponse::Ok().body(json_data)
            }
            Err(e) => {
                eprintln!("Error fetching data: {:?}", e);
                HttpResponse::InternalServerError().body("Data fetching failed")
            }
        }
    } else {
        HttpResponse::BadRequest().body("Invalid Request Body. Check your parameters.")
    }
}

#[post("/pricing/spot/forecast")]
pub async fn spot_forecast() -> impl Responder {
    HttpResponse::Ok().body("Spot Forecast")
}

#[post("/pricing/data-transfer/inter-region")]
pub async fn inter_region_data_transfer(
    pool: web::Data<Pool<Postgres>>,
    req_body: web::Json<InterRegionDataTransferRequest>,
) -> impl Responder {
    if validator::validate_inter_region_transfer_request(req_body.clone()) {
        let result = db::fetch_inter_region_data_transfer(&pool, req_body.into_inner()).await;

        match result {
            Ok(data) => {
                let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string());

                return HttpResponse::Ok().body(json_data);
            }
            Err(e) => {
                eprintln!("Error fetching data: {:?}", e);
                return HttpResponse::InternalServerError().body("Data fetching failed");
            }
        }
    } else {
        return HttpResponse::BadRequest().body("Invalid Request Body. Check your parameters.");
    }
}

#[post("/pricing/data-transfer/external")]
pub async fn external_data_transfer(
    pool: web::Data<Pool<Postgres>>,
    req_body: web::Json<ExternalDataTransferRequest>,
) -> impl Responder {
    if validator::validate_external_data_transfer_request(req_body.clone()) {
        let result = db::fetch_external_data_transfer(&pool, req_body.into_inner()).await;

        match result {
            Ok(data) => {
                let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string());

                return HttpResponse::Ok().body(json_data);
            }
            Err(e) => {
                eprintln!("Error fetching data: {:?}", e);
                return HttpResponse::InternalServerError().body("Data fetching failed");
            }
        }
    } else {
        return HttpResponse::BadRequest().body("Invalid Request Body. Check your parameters.");
    }
}
