use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BandwidthTier {
    pub from_region_code: String,
    pub start_range: u64,
    pub end_range: u64,
    pub price_per_gb: f64,
}
