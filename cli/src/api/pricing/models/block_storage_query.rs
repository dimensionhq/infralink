pub struct BlockStorageQuery {
    pub regions: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub storage_media: Option<String>,
    pub volume_api_name: Option<String>,
}
