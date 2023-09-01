use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExternalDataTransferResponse {
    pub from_region_code: String,
    pub start_range: i64,
    pub end_range: i64,
    pub price_per_gb: f64,
}
