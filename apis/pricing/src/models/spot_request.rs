use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Deserialize, Serialize, Debug, Clone)]
pub struct SpotRequest {
    pub regions: Option<Vec<String>>,
    pub availability_zones: Option<Vec<String>>,
    pub instance_types: Option<Vec<String>>,
    pub min_price_per_hour: Option<f64>,
    pub max_price_per_hour: Option<f64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
}
