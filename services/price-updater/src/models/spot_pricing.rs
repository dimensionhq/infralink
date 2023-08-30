use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotInstance {
    pub instance_type: String,
    pub spot_price: String,
}
