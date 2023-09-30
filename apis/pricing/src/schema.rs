use actix_web::{http::header::HeaderMap, web};
use anyhow::Result;
use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

use crate::{
    db,
    models::{
        block_storage_request::BlockStorageRequest, block_storage_response::BlockStorageResponse,
        external_data_transfer_request::ExternalDataTransferRequest,
        external_data_transfer_response::ExternalDataTransferResponse,
        inter_region_data_transfer_request::InterRegionDataTransferRequest,
        inter_region_data_transfer_response::InterRegionDataTransferResponse,
        on_demand_request::OnDemandRequest, on_demand_response::OnDemandResponse,
        spot_request::SpotRequest, spot_response::SpotResponse,
    },
};

pub async fn extract_and_validate_key(
    pool: web::Data<Pool<Postgres>>,
    headers: &HeaderMap,
) -> Result<()> {
    // Extract the bearer token from the header
    let bearer_token = match headers.get("authorization") {
        Some(token) => token.to_str().unwrap().replace("Bearer ", ""),
        None => {
            return Err(anyhow::anyhow!("No authorization header found"));
        }
    };

    if bearer_token != std::env::var("BOT_API_TOKEN").unwrap()
        && db::validate_api_key(&pool, &bearer_token)
            .await
            .unwrap_or(false)
            == false
    {
        return Err(anyhow::anyhow!("Invalid API key"));
    }

    Ok(())
}

pub struct Query;

#[Object]
impl Query {
    async fn on_demand(
        &self,
        ctx: &Context<'_>,
        request: OnDemandRequest,
    ) -> Result<Vec<OnDemandResponse>> {
        let pool = ctx
            .data::<web::Data<Pool<Postgres>>>()
            .expect("Failed to get the pool");

        let headers = ctx.data::<HeaderMap>().expect("Failed to get the headers");

        let key_validation = extract_and_validate_key(pool.clone(), headers).await;

        if key_validation.is_err() {
            return Err(anyhow::anyhow!(
                "Failed to validate API key. Make sure you have a valid API key. This could also be a problem with the API service. If you're sure you have a valid API key, please contact support."
            ));
        }

        let results = match db::fetch_on_demand_data(pool, request).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch on demand data: {}", e);
                return Err(anyhow::anyhow!("Failed to fetch on demand data"));
            }
        };

        if results.is_empty() {
            return Err(anyhow::anyhow!("No data found"));
        }

        Ok(results)
    }

    async fn spot(&self, ctx: &Context<'_>, request: SpotRequest) -> Result<Vec<SpotResponse>> {
        let pool = ctx
            .data::<web::Data<Pool<Postgres>>>()
            .expect("Failed to get the pool");

        let headers = ctx.data::<HeaderMap>().expect("Failed to get the headers");

        let key_validation = extract_and_validate_key(pool.clone(), headers).await;

        if key_validation.is_err() {
            return Err(anyhow::anyhow!(
                "Failed to validate API key. Make sure you have a valid API key. This could also be a problem with the API service. If you're sure you have a valid API key, please contact support."
            ));
        }

        let results = match db::fetch_spot_data(pool, request).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch spot data: {}", e);
                return Err(anyhow::anyhow!("Failed to fetch spot data"));
            }
        };

        if results.is_empty() {
            return Err(anyhow::anyhow!("No data found"));
        }

        Ok(results)
    }

    async fn inter_region_data_transfer(
        &self,
        ctx: &Context<'_>,
        request: InterRegionDataTransferRequest,
    ) -> Result<Vec<InterRegionDataTransferResponse>> {
        let pool = ctx
            .data::<web::Data<Pool<Postgres>>>()
            .expect("Failed to get the pool");

        let headers = ctx.data::<HeaderMap>().expect("Failed to get the headers");

        let key_validation = extract_and_validate_key(pool.clone(), headers).await;

        if key_validation.is_err() {
            return Err(anyhow::anyhow!(
                "Failed to validate API key. Make sure you have a valid API key. This could also be a problem with the API service. If you're sure you have a valid API key, please contact support."
            ));
        }

        let results = match db::fetch_inter_region_data_transfer(pool, request).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch inter region data transfer: {}", e);
                return Err(anyhow::anyhow!(
                    "Failed to fetch inter region data transfer"
                ));
            }
        };

        if results.is_empty() {
            return Err(anyhow::anyhow!("No data found"));
        }

        Ok(results)
    }

    async fn external_data_transfer(
        &self,
        ctx: &Context<'_>,
        request: ExternalDataTransferRequest,
    ) -> Result<Vec<ExternalDataTransferResponse>> {
        let pool = ctx
            .data::<web::Data<Pool<Postgres>>>()
            .expect("Failed to get the pool");

        let headers = ctx.data::<HeaderMap>().expect("Failed to get the headers");

        let key_validation = extract_and_validate_key(pool.clone(), headers).await;

        if key_validation.is_err() {
            return Err(anyhow::anyhow!(
                "Failed to validate API key. Make sure you have a valid API key. This could also be a problem with the API service. If you're sure you have a valid API key, please contact support."
            ));
        }

        let results = match db::fetch_external_data_transfer(pool, request).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch external data transfer: {}", e);
                return Err(anyhow::anyhow!("Failed to fetch external data transfer"));
            }
        };

        if results.is_empty() {
            return Err(anyhow::anyhow!("No data found"));
        }

        Ok(results)
    }

    async fn block_storage(
        &self,
        ctx: &Context<'_>,
        request: BlockStorageRequest,
    ) -> Result<Vec<BlockStorageResponse>> {
        let pool = ctx
            .data::<web::Data<Pool<Postgres>>>()
            .expect("Failed to get the pool");

        let headers = ctx.data::<HeaderMap>().expect("Failed to get the headers");

        let key_validation = extract_and_validate_key(pool.clone(), headers).await;

        if key_validation.is_err() {
            return Err(anyhow::anyhow!(
                "Failed to validate API key. Make sure you have a valid API key. This could also be a problem with the API service. If you're sure you have a valid API key, please contact support."
            ));
        }

        let results = match db::fetch_storage(pool, request).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to fetch block storage data: {}", e);
                return Err(anyhow::anyhow!("Failed to fetch block storage data"));
            }
        };

        if results.is_empty() {
            return Err(anyhow::anyhow!("No data found"));
        }

        Ok(results)
    }
}
