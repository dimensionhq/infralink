use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StoragePricing {
    pub region: String,
    pub volume_api_name: String,
    pub storage_media: String,
    pub price_per_gb_month: f64,
}
