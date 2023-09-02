use crate::models::external_data_transfer_request::ExternalDataTransferRequest;
use crate::models::external_data_transfer_response::ExternalDataTransferResponse;
use crate::models::inter_region_data_transfer_request::InterRegionDataTransferRequest;
use crate::models::inter_region_data_transfer_response::InterRegionDataTransferResponse;
use crate::models::on_demand_response::OnDemandResponse;
use crate::models::spot_response::SpotResponse;
use crate::models::storage_request::StorageRequest;
use crate::models::storage_response::StorageResponse;
use crate::models::{on_demand_request::OnDemandRequest, spot_request::SpotRequest};
use actix_web::web;
use anyhow::Result;
use sqlx::{Pool, Postgres, QueryBuilder};

pub async fn create_pool(database_url: &str) -> Pool<Postgres> {
    Pool::connect(database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn fetch_on_demand_data(
    pool: &web::Data<Pool<Postgres>>,
    req: OnDemandRequest,
) -> Result<Vec<OnDemandResponse>> {
    let regions = req.regions.clone();
    let instance_types = req.instance_types.clone();

    let mut query = QueryBuilder::new("SELECT * FROM on_demand WHERE 1=1");

    // Handle regions
    if let Some(ref regions) = regions {
        query.push(" AND region IN (");
        let mut separated = query.separated(", ");
        for region in regions.iter() {
            separated.push_bind(region);
        }
        separated.push_unseparated(") ");
    }

    // Handle instance types
    if let Some(ref instance_types) = instance_types {
        query.push("AND instance_type IN (");
        let mut separated = query.separated(", ");
        for instance_type in instance_types.iter() {
            separated.push_bind(instance_type);
        }
        separated.push_unseparated(") ");
    }

    // Handle remaining filters
    if let Some(min_vcpu) = req.min_vcpu {
        query.push(&format!(" AND vcpu_count >= {}", min_vcpu));
    }
    if let Some(max_vcpu) = req.max_vcpu {
        query.push(&format!(" AND vcpu_count <= {}", max_vcpu));
    }
    if let Some(min_memory) = req.min_memory {
        query.push(&format!(" AND memory >= {}", min_memory));
    }
    if let Some(max_memory) = req.max_memory {
        query.push(&format!(" AND memory <= {}", max_memory));
    }
    if let Some(min_price_per_hour) = req.min_price_per_hour {
        query.push(&format!(" AND price_per_hour >= {}", min_price_per_hour));
    }
    if let Some(max_price_per_hour) = req.max_price_per_hour {
        query.push(&format!(" AND price_per_hour <= {}", max_price_per_hour));
    }
    if let Some(sort_by) = req.sort_by {
        query.push(&format!(" ORDER BY {}", sort_by));
    }
    if let Some(sort_order) = req.sort_order {
        query.push(&format!(" {}", sort_order));
    }
    if let Some(limit) = req.limit {
        query.push(&format!(" LIMIT {}", limit));
    }

    // Execute the query
    let result = query.build_query_as().fetch_all(&***pool).await.unwrap();

    Ok(result)
}

pub async fn fetch_spot_data(
    pool: &web::Data<Pool<Postgres>>,
    req: SpotRequest,
) -> Result<Vec<SpotResponse>> {
    let regions = req.regions.clone();
    let availability_zones = req.availability_zones.clone();
    let instance_types = req.instance_types.clone();

    let mut query = QueryBuilder::new("SELECT * FROM spot WHERE 1=1");

    // Handle regions
    if let Some(ref regions) = regions {
        query.push("AND region IN (");
        let mut separated = query.separated(", ");
        for region in regions.iter() {
            separated.push_bind(region);
        }
        separated.push_unseparated(") ");
    }

    // Handle availability zones
    if let Some(ref availability_zones) = availability_zones {
        query.push("AND availability_zone IN (");
        let mut separated = query.separated(", ");
        for availability_zone in availability_zones.iter() {
            separated.push_bind(availability_zone);
        }
        separated.push_unseparated(") ");
    }

    // Handle instance types
    if let Some(ref instance_types) = instance_types {
        query.push("AND instance_type IN (");
        let mut separated = query.separated(", ");
        for instance_type in instance_types.iter() {
            separated.push_bind(instance_type);
        }
        separated.push_unseparated(") ");
    }

    // Handle remaining filters
    if let Some(min_price_per_hour) = req.min_price_per_hour {
        query.push(&format!(" AND price_per_hour >= {}", min_price_per_hour));
    }
    if let Some(max_price_per_hour) = req.max_price_per_hour {
        query.push(&format!(" AND price_per_hour <= {}", max_price_per_hour));
    }
    if let Some(sort_by) = req.sort_by {
        query.push(&format!(" ORDER BY {}", sort_by));
    }
    if let Some(sort_order) = req.sort_order {
        query.push(&format!(" {}", sort_order));
    }
    if let Some(limit) = req.limit {
        query.push(&format!(" LIMIT {}", limit));
    }

    // Execute the query
    let result = query.build_query_as().fetch_all(&***pool).await.unwrap();

    Ok(result)
}

pub async fn fetch_external_data_transfer(
    pool: &web::Data<Pool<Postgres>>,
    data_transfer_request: ExternalDataTransferRequest,
) -> Result<Vec<ExternalDataTransferResponse>> {
    let mut query = QueryBuilder::new("SELECT * FROM external_data_transfer WHERE 1=1");

    // Handle from_region_code
    if let Some(ref from_region_code) = data_transfer_request.from_region_code {
        query.push(" AND from_region_code = ");
        query.push_bind(from_region_code);
    }

    // Handle start_range if it exists
    if let Some(start_range) = data_transfer_request.start_range {
        query.push(" AND start_range <= ");
        query.push_bind(start_range);
    }

    // Handle sort_by
    if let Some(ref sort_by) = data_transfer_request.sort_by {
        query.push(" ORDER BY ");
        query.push_bind(sort_by);
    }

    // Handle sort_order
    if let Some(ref sort_order) = data_transfer_request.sort_order {
        query.push(" ");
        query.push_bind(sort_order);
    }

    // Execute the query
    let rows: Vec<ExternalDataTransferResponse> =
        query.build_query_as().fetch_all(&***pool).await?;

    Ok(rows)
}

pub async fn fetch_inter_region_data_transfer(
    pool: &web::Data<Pool<Postgres>>,
    data_transfer_request: InterRegionDataTransferRequest,
) -> Result<Vec<InterRegionDataTransferResponse>, anyhow::Error> {
    let mut query = QueryBuilder::new("SELECT * FROM inter_region_data_transfer WHERE 1=1");

    // Handle from_region_code
    if data_transfer_request.from_region_code.as_ref().is_some() {
        query.push(" AND from_region_code = ");
        query.push_bind(&data_transfer_request.from_region_code);
    }

    // Handle to_region_code
    if data_transfer_request.to_region_code.as_ref().is_some() {
        query.push(" AND to_region_code = ");
        query.push_bind(&data_transfer_request.to_region_code);
    }

    // Handle sort_by
    if data_transfer_request.sort_by.as_ref().is_some() {
        query.push(" ORDER BY ");
        query.push_bind(&data_transfer_request.sort_by);
    }

    // Handle sort_order
    if data_transfer_request.sort_order.as_ref().is_some() {
        query.push(" ");
        query.push_bind(&data_transfer_request.sort_order);
    }

    // Execute the query
    let rows: Vec<InterRegionDataTransferResponse> =
        query.build_query_as().fetch_all(&***pool).await?;

    Ok(rows)
}

pub async fn fetch_storage(
    pool: &web::Data<Pool<Postgres>>,
    storage_request: StorageRequest,
) -> Result<Vec<StorageResponse>> {
    let mut query = QueryBuilder::new("SELECT * FROM storage WHERE 1=1");

    // Handle region
    if let Some(ref regions) = storage_request.regions {
        query.push("AND region IN (");
        let mut separated = query.separated(", ");
        for region in regions.iter() {
            separated.push_bind(region);
        }
        separated.push_unseparated(") ");
    }

    // Handle volume_api_name
    if storage_request.volume_api_name.as_ref().is_some() {
        query.push(" AND volume_api_name = ");
        query.push_bind(&storage_request.volume_api_name);
    }

    // Handle storage_media
    if storage_request.storage_media.as_ref().is_some() {
        query.push(" AND storage_media = ");
        query.push_bind(&storage_request.storage_media);
    }

    // Handle sort_by
    if storage_request.sort_by.as_ref().is_some() {
        query.push(" ORDER BY ");
        query.push_bind(&storage_request.sort_by);
    }

    // Handle sort_order
    if storage_request.sort_order.as_ref().is_some() {
        query.push(" ");
        query.push_bind(&storage_request.sort_order);
    }

    // Execute the query
    let rows: Vec<StorageResponse> = query.build_query_as().fetch_all(&***pool).await.unwrap();

    Ok(rows)
}
