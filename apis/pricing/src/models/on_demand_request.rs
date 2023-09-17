use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, InputObject)]
#[serde(rename_all = "kebab-case")]
pub struct OnDemandRequest {
    pub regions: Option<Vec<String>>,
    pub instance_types: Option<Vec<String>>,
    pub min_vcpu: Option<f64>,
    pub max_vcpu: Option<f64>,
    pub min_memory: Option<f64>,
    pub max_memory: Option<f64>,
    pub min_price_per_hour: Option<f64>,
    pub max_price_per_hour: Option<f64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
}
