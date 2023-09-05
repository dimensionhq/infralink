use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, GraphQLObject, sqlx::FromRow)]
pub struct InterRegionDataTransferResponse {
    pub from_region_code: String,
    pub to_region_code: String,
    pub price_per_gb: f64,
}
