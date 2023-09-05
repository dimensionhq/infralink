use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLInputObject)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalDataTransferRequest {
    pub from_region_code: Option<String>,
    pub start_range: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
