pub struct SpotQuery {
    pub availability_zones: Option<Vec<String>>,
    pub instance_types: Option<Vec<String>>,
    pub max_price_per_hour: Option<f32>,
    pub min_price_per_hour: Option<f32>,
    pub regions: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
}
