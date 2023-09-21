use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotResponse {
    pub availability_zone: String,
    pub instance_type: String,
    pub price_per_hour: f64,
    pub region: String,
}
