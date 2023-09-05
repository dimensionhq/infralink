use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, GraphQLInputObject)]
#[serde(rename_all = "kebab-case")]
pub struct InterRegionDataTransferRequest {
    pub from_region_code: Option<String>,
    pub to_region_code: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
