use async_graphql::Object;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OnDemandResponse {
    pub region: String,
    pub instance_type: String,
    pub architecture: String,
    pub vcpu_count: f64,
    pub memory: f64,
    pub price_per_hour: f64,
}

#[Object]
impl OnDemandResponse {
    async fn region(&self) -> &str {
        &self.region
    }

    async fn instance_type(&self) -> &str {
        &self.instance_type
    }

    async fn architecture(&self) -> &str {
        &self.architecture
    }

    async fn vcpu_count(&self) -> f64 {
        self.vcpu_count
    }

    async fn memory(&self) -> f64 {
        self.memory
    }

    async fn price_per_hour(&self) -> f64 {
        self.price_per_hour
    }
}
