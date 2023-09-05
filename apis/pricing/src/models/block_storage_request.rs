use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLInputObject)]
pub struct BlockStorageRequest {
    pub regions: Option<Vec<String>>,
    pub volume_api_name: Option<Vec<String>>,
    pub storage_media: Option<Vec<String>>,
}
