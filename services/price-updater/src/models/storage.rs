use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Storage {
    pub region: String,
    pub storage_media: String,
    pub volume_api_name: String,
    pub price_per_gb_month: f32,
}
