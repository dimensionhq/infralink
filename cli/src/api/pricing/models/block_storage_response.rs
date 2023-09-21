use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStorageResponse {
    pub price_per_gb_month: f64,
    pub region: String,
    pub storage_media: String,
    pub volume_api_name: String,
}
