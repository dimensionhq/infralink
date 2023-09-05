use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Debug, Serialize, GraphQLObject, sqlx::FromRow)]
pub struct OnDemandResponse {
    pub region: String,
    pub instance_type: String,
    pub architecture: String,
    pub vcpu_count: f64,
    pub memory: f64,
    pub price_per_hour: f64,
}
