use actix_web::web::Data;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};
use sqlx::{Pool, Postgres};

use crate::models::{
    on_demand_request::OnDemandRequest, on_demand_response::OnDemandResponse,
    spot_request::SpotRequest, spot_response::SpotResponse,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Context {
    pub db_pool: Data<Pool<Postgres>>,
}

pub struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    fn on_demand(request: OnDemandRequest) -> FieldResult<OnDemandResponse> {
        println!("On-demand Request: {:?}", request);

        // Fetch from DB and return
        Ok(OnDemandResponse {
            region: String::new(),
            price_per_hour: 0.0,
            instance_type: String::new(),
            architecture: String::from("arm64"),
            vcpu_count: 1.0,
            memory: 1.0,
        })
    }

    fn spot(request: SpotRequest) -> FieldResult<SpotResponse> {
        println!("Spot Request: {:?}", request);

        // Fetch from DB and return
        Ok(SpotResponse {
            availability_zone: String::new(),
            region: String::new(),
            price_per_hour: 0.0,
            instance_type: String::new(),
        })
    }
}
