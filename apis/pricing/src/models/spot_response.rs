use async_graphql::Object;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SpotResponse {
    pub availability_zone: String,
    pub region: String,
    pub instance_type: String,
    pub price_per_hour: f64,
}

#[Object]
impl SpotResponse {
    async fn availability_zone(&self) -> &str {
        &self.availability_zone
    }

    async fn region(&self) -> &str {
        &self.region
    }

    async fn instance_type(&self) -> &str {
        &self.instance_type
    }

    async fn price_per_hour(&self) -> f64 {
        self.price_per_hour
    }
}
