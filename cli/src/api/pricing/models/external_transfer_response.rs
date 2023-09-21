use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDataTransferResponse {
    pub end_range: i64,
    pub from_region_code: String,
    pub price_per_gb: f64,
    pub start_range: i64,
}
