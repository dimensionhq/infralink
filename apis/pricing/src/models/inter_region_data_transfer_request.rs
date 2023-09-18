use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Deserialize, Serialize, Debug, Clone)]
pub struct InterRegionDataTransferRequest {
    pub from_region_code: Option<String>,
    pub to_region_code: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
