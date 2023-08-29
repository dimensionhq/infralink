use crate::models::on_demand_response::OnDemandResponse;
use crate::models::spot_response::SpotResponse;
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

    let mut query = QueryBuilder::new("SELECT * FROM on_demand WHERE ");

    // Handle regions
    if let Some(ref regions) = regions {
        query.push("region IN (");
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

    let mut query = QueryBuilder::new("SELECT * FROM spot WHERE ");

    // Handle regions
    if let Some(ref regions) = regions {
        query.push("region IN (");
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
