use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OnDemandResponse {
    pub region: String,
    pub instance_type: String,
    pub vcpu_count: f64,
    pub memory: f64,
    pub price_per_hour: f64,
}
