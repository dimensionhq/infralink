use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockStorageResponse {
    pub regions: Option<Vec<String>>,
    pub volume_api_name: Option<Vec<String>>,
    pub storage_media: Option<Vec<String>>,
    pub price_per_gb_month: Option<f64>,
}
