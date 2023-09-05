use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, GraphQLInputObject)]
#[serde(rename_all = "kebab-case")]
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
