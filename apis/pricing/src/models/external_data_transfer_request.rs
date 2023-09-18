use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalDataTransferRequest {
    pub from_region_code: Option<String>,
    pub start_range: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
