use std::collections::HashMap;

use crate::{api::pricing, models::region::AwsRegion};

async fn calculate_deployment(
    regions: Vec<AwsRegion>,
    control_plane_specs: Option<(f64, f64)>,
    spot_control_plane: bool,
    node_specs: (f64, f64),
    storage_size_gb: f64,
    node_count: f64,
    outbound_data_gb: u64,
) -> HashMap<AwsRegion, f64> {
    let control_plane_options = if spot_control_plane {
        pricing::api::get_cheapest_spot_instances(Some(regions.clone()))
            .await
            .unwrap()
    } else {
        pricing::api::get_cheapest_on_demand_instances(
            Some(regions.clone()),
            Some(control_plane_specs.unwrap().0),
            Some(control_plane_specs.unwrap().1),
        )
        .await
        .unwrap()
    };

    let node_options = pricing::api::get_cheapest_on_demand_instances(
        Some(regions.clone()),
        Some(node_specs.0),
        Some(node_specs.1),
    )
    .await
    .unwrap();

    let storage_cost = pricing::api::get_storage_pricing(
        Some(regions.clone()),
        Some("SSD".to_string()),
        Some("gp3".to_string()),
        storage_size_gb,
    )
    .await
    .unwrap();

    let outbound_data_cost =
        pricing::api::get_external_bandwidth_pricing(Some(regions), outbound_data_gb)
            .await
            .unwrap();

    let mut total_deployment_cost: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, control_plane_instance) in control_plane_options {
        let node_instance = node_options.get(&region).unwrap();

        let storage = storage_cost.get(&region).unwrap();
        let outbound_data = outbound_data_cost.get(&region).unwrap();

        let instance_cost =
            control_plane_instance.price_per_hour + (node_count * node_instance.price_per_hour);

        // Calculate the monthly cost
        let monthly_cost =
            (instance_cost * 730.0) + ((node_count + 1.0) * (storage["gp3"])) + outbound_data;

        let rounded_monthly_cost: f64;

        if spot_control_plane {
            rounded_monthly_cost = (monthly_cost * 100.0).round() / 100.0;
        } else {
            rounded_monthly_cost = (monthly_cost * 10.0).round() / 10.0;
        }

        total_deployment_cost.insert(region, rounded_monthly_cost);
    }

    total_deployment_cost
}

pub async fn calculate_cheapest_deployment(regions: Vec<AwsRegion>) -> HashMap<AwsRegion, f64> {
    calculate_deployment(regions, None, true, (1.0, 1.0), 8.0, 1.0, 0).await
}

pub async fn calculate_large_deployment(regions: Vec<AwsRegion>) -> HashMap<AwsRegion, f64> {
    calculate_deployment(
        regions,
        Some((2.0, 4.0)),
        false,
        (4.0, 8.0),
        50.0,
        3.0,
        1000,
    )
    .await
}
