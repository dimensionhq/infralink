use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Debug, Serialize, GraphQLObject, sqlx::FromRow)]
pub struct SpotResponse {
    pub availability_zone: String,
    pub region: String,
    pub instance_type: String,
    pub price_per_hour: f64,
}
