use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct InterRegionDataTransferResponse {
    pub from_region_code: String,
    pub to_region_code: String,
    pub price_per_gb: f64,
}

#[Object]
impl InterRegionDataTransferResponse {
    pub async fn from_region_code(&self) -> &str {
        &self.from_region_code
    }

    pub async fn to_region_code(&self) -> &str {
        &self.to_region_code
    }

    pub async fn price_per_gb(&self) -> f64 {
        self.price_per_gb
    }
}
