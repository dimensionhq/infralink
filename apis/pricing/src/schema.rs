use async_graphql::Object;

use crate::models::{on_demand_request::OnDemandRequest, on_demand_response::OnDemandResponse};

struct Query;

#[Object]
impl Query {
    async fn on_demand(&self, request: OnDemandRequest) -> OnDemandResponse {
        println!("{:?}", request);

        OnDemandResponse {
            price_per_hour: 0.0,
            region: String::new(),
            instance_type: String::new(),
            architecture: String::new(),
            vcpu_count: 0.0,
            memory: 0.0,
        }
    }
}
