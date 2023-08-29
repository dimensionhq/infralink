use crate::models::on_demand_response::OnDemandResponse;
use crate::models::spot_response::SpotResponse;
use crate::models::{on_demand_request::OnDemandRequest, spot_request::SpotRequest};
use actix_web::web;
use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn create_pool(database_url: &str) -> Pool<Postgres> {
    Pool::connect(database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn fetch_on_demand_data(
    pool: &web::Data<Pool<Postgres>>,
    req: OnDemandRequest,
) -> Result<Vec<OnDemandResponse>> {
    let mut query = r#"SELECT * from on_demand"#.to_string();

    if let Some(regions) = req.regions {
        if !regions.is_empty() {
            let regions = regions.join("','");
            query.push_str(&format!(" WHERE region IN ('{}')", regions));
        }
    }

    if let Some(instance_types) = req.instance_types {
        if !instance_types.is_empty() {
            let instance_types = instance_types.join("','");
            query.push_str(&format!(" AND instance_type IN ('{}')", instance_types));
        }
    }

    if let Some(min_vcpu) = req.min_vcpu {
        query.push_str(&format!(" AND vcpu_count >= {}", min_vcpu));
    }

    if let Some(max_vcpu) = req.max_vcpu {
        query.push_str(&format!(" AND vcpu_count <= {}", max_vcpu));
    }

    if let Some(min_memory) = req.min_memory {
        query.push_str(&format!(" AND memory >= {}", min_memory));
    }

    if let Some(max_memory) = req.max_memory {
        query.push_str(&format!(" AND memory <= {}", max_memory));
    }

    if let Some(min_price_per_hour) = req.min_price_per_hour {
        query.push_str(&format!(" AND price_per_hour >= {}", min_price_per_hour));
    }

    if let Some(max_price_per_hour) = req.max_price_per_hour {
        query.push_str(&format!(" AND price_per_hour <= {}", max_price_per_hour));
    }

    if let Some(sort_by) = req.sort_by {
        query.push_str(&format!(" ORDER BY {} ", sort_by));
    }

    if let Some(sort_order) = req.sort_order {
        query.push_str(&format!("{} ", sort_order));
    }

    if let Some(limit) = req.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    // Execute the query
    let result = sqlx::query_as::<_, OnDemandResponse>(&query)
        .fetch_all(pool.as_ref())
        .await?;

    Ok(result)
}

pub async fn fetch_spot_data(
    pool: &web::Data<Pool<Postgres>>,
    req: SpotRequest,
) -> Result<Vec<SpotResponse>> {
    let mut query = r#"SELECT * from spot"#.to_string();

    if let Some(regions) = req.regions {
        if !regions.is_empty() {
            let regions = regions.join("','");
            query.push_str(&format!(" WHERE region IN ('{}')", regions));
        }
    }

    if let Some(availability_zones) = req.availability_zones {
        if !availability_zones.is_empty() {
            let availability_zones = availability_zones.join("','");

            query.push_str(&format!(
                " AND availability_zone IN ('{}')",
                availability_zones
            ));
        }
    }

    if let Some(instance_types) = req.instance_types {
        if !instance_types.is_empty() {
            let instance_types = instance_types.join("','");
            query.push_str(&format!(" AND instance_type IN ('{}')", instance_types));
        }
    }

    if let Some(min_price_per_hour) = req.min_price_per_hour {
        query.push_str(&format!(" AND price_per_hour >= {}", min_price_per_hour));
    }

    if let Some(max_price_per_hour) = req.max_price_per_hour {
        query.push_str(&format!(" AND price_per_hour <= {}", max_price_per_hour));
    }

    if let Some(sort_by) = req.sort_by {
        query.push_str(&format!(" ORDER BY {} ", sort_by));
    }

    if let Some(sort_order) = req.sort_order {
        query.push_str(&format!("{} ", sort_order));
    }

    if let Some(limit) = req.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    // Execute the query
    let result = sqlx::query_as::<_, SpotResponse>(&query)
        .fetch_all(pool.as_ref())
        .await?;

    Ok(result)
}
