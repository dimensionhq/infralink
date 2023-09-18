use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct BlockStorageRequest {
    pub regions: Option<Vec<String>>,
    pub volume_api_name: Option<String>,
    pub storage_media: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
