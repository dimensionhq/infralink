use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterRegionDataTransferResponse {
    pub from_region_code: String,
    pub price_per_gb: f64,
    pub to_region_code: String,
}
