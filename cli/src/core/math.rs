use std::collections::HashMap;

use crate::{api::pricing, models::region::AwsRegion};

pub async fn calculate_cheapest_deployment(regions: Vec<AwsRegion>) -> HashMap<AwsRegion, f64> {
    let control_plane_options = pricing::api::get_cheapest_spot_instances(Some(regions.clone()))
        .await
        .unwrap();

    let node_options =
        pricing::api::get_cheapest_on_demand_instances(Some(regions.clone()), Some(1.0), Some(1.0))
            .await
            .unwrap();

    // calculate the cost of 8 GB of EBS storage
    let storage_cost = pricing::api::get_storage_pricing(
        Some(regions),
        Some("SSD".to_string()),
        Some("gp3".to_string()),
        8.0,
    )
    .await
    .unwrap();

    let mut cheapest_deployment: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, control_plane_instance) in control_plane_options {
        let node_instance = node_options.get(&region).unwrap();
        let storage = storage_cost.get(&region).unwrap();

        let total_cost = control_plane_instance.price_per_hour + node_instance.price_per_hour;

        // multiply the total cost by 730 to get the monthly cost
        let total_cost = (total_cost * 730.0) + (2.0 * storage["gp3"]);

        // round the total cost to 1 decimal place
        let total_cost = (total_cost * 10.0).round() / 10.0;

        cheapest_deployment.insert(region, total_cost);
    }

    cheapest_deployment
}

pub async fn calculate_large_deployment(regions: Vec<AwsRegion>) -> HashMap<AwsRegion, f64> {
    let control_plane_options =
        pricing::api::get_cheapest_on_demand_instances(Some(regions.clone()), Some(2.0), Some(4.0))
            .await
            .unwrap();

    let node_options =
        pricing::api::get_cheapest_on_demand_instances(Some(regions.clone()), Some(4.0), Some(8.0))
            .await
            .unwrap();

    // calculate the cost of 50 GB of EBS storage
    let storage_cost = pricing::api::get_storage_pricing(
        Some(regions.clone()),
        Some("SSD".to_string()),
        Some("gp3".to_string()),
        50.0,
    )
    .await
    .unwrap();

    // calculat the cost of 1 TB of outbound data transfer
    let outbound_data_cost = pricing::api::get_external_bandwidth_pricing(Some(regions), 1000)
        .await
        .unwrap();

    let mut large_deployment: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, control_plane_instance) in control_plane_options {
        let node_instance = node_options.get(&region).unwrap();
        let storage = storage_cost.get(&region).unwrap();
        let outbound_data = outbound_data_cost.get(&region).unwrap();

        let total_cost =
            control_plane_instance.price_per_hour + (3.0 * node_instance.price_per_hour);

        // multiply the total cost by 730 to get the monthly cost
        let total_cost = (total_cost * 730.0) + (50.0 * storage["gp3"]) + outbound_data;

        // round the total cost to 1 decimal place
        let total_cost = (total_cost * 10.0).round() / 10.0;

        large_deployment.insert(region, total_cost);
    }

    large_deployment
}
