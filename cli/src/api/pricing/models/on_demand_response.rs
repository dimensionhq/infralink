use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnDemandResponse {
    pub architecture: String,
    pub instance_type: String,
    pub memory: f64,
    pub price_per_hour: f64,
    pub region: String,
    pub vcpu_count: f64,
}
