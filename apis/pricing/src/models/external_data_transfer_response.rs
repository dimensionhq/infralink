use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, GraphQLObject, sqlx::FromRow)]
pub struct ExternalDataTransferResponse {
    pub from_region_code: String,
    pub start_range: i32,
    pub end_range: i32,
    pub price_per_gb: f64,
}
