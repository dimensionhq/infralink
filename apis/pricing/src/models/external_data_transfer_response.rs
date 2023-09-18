use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExternalDataTransferResponse {
    pub from_region_code: String,
    pub start_range: i64,
    pub end_range: i64,
    pub price_per_gb: f64,
}

#[Object]
impl ExternalDataTransferResponse {
    pub async fn from_region_code(&self) -> &str {
        &self.from_region_code
    }

    pub async fn start_range(&self) -> i64 {
        self.start_range
    }

    pub async fn end_range(&self) -> i64 {
        self.end_range
    }

    pub async fn price_per_gb(&self) -> f64 {
        self.price_per_gb
    }
}
