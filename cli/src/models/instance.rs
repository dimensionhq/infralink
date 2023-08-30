use serde::{Deserialize, Serialize};

use super::region::AwsRegion;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AwsSpotInstance {
    pub region: AwsRegion,
    pub availability_zone: String,
    pub instance_type: String,
    pub price_per_hour: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AwsInstance {
    pub region: AwsRegion,
    pub instance_type: String,
    pub price_per_hour: f64,
}
