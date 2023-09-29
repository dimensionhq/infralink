#[derive(Debug, Clone)]
pub struct OnDemandQuery {
    pub instance_types: Option<Vec<String>>,
    pub max_memory: Option<f32>,
    pub max_price_per_hour: Option<f32>,
    pub max_vcpu: Option<u32>,
    pub min_memory: Option<f32>,
    pub min_price_per_hour: Option<f32>,
    pub min_vcpu: Option<u32>,
    pub regions: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i32>,
}
