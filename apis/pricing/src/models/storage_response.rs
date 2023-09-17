use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct StorageResponse {
    pub region: Option<String>,
    pub volume_api_name: Option<String>,
    pub storage_media: Option<String>,
    pub price_per_gb_month: Option<f64>,
}
