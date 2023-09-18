use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct BlockStorageResponse {
    pub region: Option<String>,
    pub volume_api_name: Option<String>,
    pub storage_media: Option<String>,
    pub price_per_gb_month: Option<f64>,
}

#[Object]
impl BlockStorageResponse {
    async fn region(&self) -> Option<String> {
        self.region.clone()
    }

    async fn volume_api_name(&self) -> Option<String> {
        self.volume_api_name.clone()
    }

    async fn storage_media(&self) -> Option<String> {
        self.storage_media.clone()
    }

    async fn price_per_gb_month(&self) -> Option<f64> {
        self.price_per_gb_month
    }
}
