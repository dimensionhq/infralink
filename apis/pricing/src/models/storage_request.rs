use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct StorageRequest {
    pub regions: Option<Vec<String>>,
    pub volume_api_name: Option<String>,
    pub storage_media: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
