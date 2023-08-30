use std::collections::HashMap;

use crate::{api::pricing, models::region::AwsRegion};

pub async fn calculate_cheapest_deployment(regions: Vec<AwsRegion>) -> HashMap<AwsRegion, f64> {
    let control_plane_cost = pricing::api::get_cheapest_spot_instances(Some(regions.clone()))
        .await
        .unwrap();

    let node_cost =
        pricing::api::get_cheapest_on_demand_instances(Some(regions), Some(1.0), Some(1.0))
            .await
            .unwrap();

    // calculate the cost of EBS depending on the region

    let mut cheapest_deployment: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, control_plane_instance) in control_plane_cost {
        let node_instance = node_cost.get(&region).unwrap();

        let total_cost = control_plane_instance.price_per_hour + node_instance.price_per_hour;

        // multiply the total cost by 730 to get the monthly cost
        let total_cost = total_cost * 730.0;

        // round the total cost to 1 decimal place
        let total_cost = (total_cost * 10.0).round() / 10.0;

        cheapest_deployment.insert(region, total_cost);
    }

    cheapest_deployment
}
