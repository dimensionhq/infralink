use std::collections::HashMap;
use std::str::FromStr;

use constants::regions::UNSUPPORTED_REGIONS;
use indexmap::IndexMap;
use pricing_sdk::BlockStorageQuery;
use pricing_sdk::ExternalDataTransferQuery;
use pricing_sdk::OnDemandQuery;
use pricing_sdk::PricingQuery;
use pricing_sdk::SpotQuery;
use types::deployment_configuration::DeploymentConfiguration;
use types::region::AwsRegion;

pub async fn calculate_deployment(
    deployment_configuration: DeploymentConfiguration,
) -> HashMap<AwsRegion, IndexMap<String, f64>> {
    let mut query = PricingQuery::start();

    // load spot & on-demand options for the control plane
    if deployment_configuration.spot_control_plane {
        query.with_spot(
            None,
            SpotQuery {
                availability_zones: None,
                instance_types: None,
                max_price_per_hour: None,
                min_price_per_hour: None,
                regions: Some(
                    deployment_configuration
                        .regions
                        .clone()
                        .into_iter()
                        .map(|r| r.code())
                        .collect(),
                ),
                sort_by: Some(String::from("price_per_hour")),
                sort_order: Some(String::from("asc")),
                limit: None,
            },
        );
    } else {
        query.with_on_demand(
            Some(String::from("controlPlaneOnDemand")),
            OnDemandQuery {
                instance_types: None,
                max_memory: None,
                max_price_per_hour: None,
                max_vcpu: None,
                min_memory: Some(deployment_configuration.control_plane_specs.unwrap().1),
                min_price_per_hour: None,
                min_vcpu: Some(deployment_configuration.control_plane_specs.unwrap().0),
                regions: Some(
                    deployment_configuration
                        .regions
                        .clone()
                        .into_iter()
                        .map(|r| r.code())
                        .collect(),
                ),
                sort_by: Some(String::from("price_per_hour")),
                sort_order: Some(String::from("asc")),
                limit: None,
            },
        );
    };

    // load options for the nodes
    query.with_on_demand(
        Some(String::from("nodeOnDemand")),
        OnDemandQuery {
            instance_types: None,
            max_memory: None,
            max_price_per_hour: None,
            max_vcpu: None,
            min_memory: Some(deployment_configuration.node_specs.1),
            min_price_per_hour: None,
            min_vcpu: Some(deployment_configuration.node_specs.0),
            regions: Some(
                deployment_configuration
                    .regions
                    .clone()
                    .into_iter()
                    .map(|r| r.code())
                    .collect(),
            ),
            sort_by: Some(String::from("price_per_hour")),
            sort_order: Some(String::from("asc")),
            limit: None,
        },
    );

    // load options for the storage
    query.with_block_storage(BlockStorageQuery {
        regions: Some(
            deployment_configuration
                .regions
                .clone()
                .into_iter()
                .map(|r| r.code())
                .collect(),
        ),
        sort_by: Some(String::from("price_per_gb_month")),
        sort_order: Some(String::from("asc")),
        storage_media: Some(String::from("SSD")),
        volume_api_name: None,
    });

    // load options for the outbound data transfer
    query.with_external_data_transfer(ExternalDataTransferQuery {
        from_region_code: None,
        sort_by: None,
        sort_order: None,
        start_range: Some(deployment_configuration.storage_size_gb as i32),
    });

    let mut total_deployment_cost: HashMap<AwsRegion, IndexMap<String, f64>> = HashMap::new();

    query.end();

    let result = query.execute().await.unwrap();

    let mut control_plane_cost: HashMap<AwsRegion, f64> = HashMap::new();

    // calculate and populate the cost of the control plane
    if deployment_configuration.spot_control_plane {
        let spot_options = result.data.spot.unwrap();

        let mut cheapest_spot_in_region: HashMap<AwsRegion, f64> = HashMap::new();

        for spot in spot_options {
            let region = AwsRegion::from_str(&spot.region).unwrap();
            let current_price = cheapest_spot_in_region.get(&region);

            if current_price.is_none() || *current_price.unwrap() > spot.price_per_hour {
                cheapest_spot_in_region.insert(region, spot.price_per_hour);
            }
        }

        for (region, price) in cheapest_spot_in_region {
            control_plane_cost.insert(region, price);
        }
    } else {
        let on_demand_options = result.data.control_plane_on_demand.as_ref().unwrap();

        let mut cheapest_on_demand_in_region: HashMap<AwsRegion, f64> = HashMap::new();

        for on_demand in on_demand_options {
            if on_demand.price_per_hour == 0.0 {
                continue;
            }

            let region = AwsRegion::from_str(&on_demand.region).unwrap();
            let current_price = cheapest_on_demand_in_region.get(&region);

            if current_price.is_none() || *current_price.unwrap() > on_demand.price_per_hour {
                cheapest_on_demand_in_region.insert(region, on_demand.price_per_hour);
            }
        }

        for (region, price) in cheapest_on_demand_in_region {
            control_plane_cost.insert(region, price);
        }
    }

    let mut node_cost: HashMap<AwsRegion, f64> = HashMap::new();

    // calculate and populate the cost of the nodes per-region
    let on_demand_options = result.data.node_on_demand.as_ref().unwrap();

    let mut cheapest_node_in_region: HashMap<AwsRegion, f64> = HashMap::new();

    for on_demand in on_demand_options {
        let region = AwsRegion::from_str(&on_demand.region).unwrap();
        let current_price = cheapest_node_in_region.get(&region);

        if on_demand.price_per_hour == 0.0 {
            continue;
        }

        if current_price.is_none() || *current_price.unwrap() > on_demand.price_per_hour {
            cheapest_node_in_region.insert(region, on_demand.price_per_hour);
        }
    }

    for (region, price) in cheapest_node_in_region {
        node_cost.insert(region, price);
    }

    // calculate and populate the cost of the storage per-region
    let block_storage_options = result.data.block_storage.as_ref().unwrap();

    let mut cheapest_storage_in_region: HashMap<AwsRegion, f64> = HashMap::new();

    for block_storage in block_storage_options {
        let region = AwsRegion::from_str(&block_storage.region).unwrap();
        let current_price = cheapest_storage_in_region.get(&region);

        if current_price.is_none() || *current_price.unwrap() > block_storage.price_per_gb_month {
            cheapest_storage_in_region.insert(region, block_storage.price_per_gb_month);
        }
    }

    let mut storage_cost: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, price) in cheapest_storage_in_region {
        storage_cost.insert(region, price);
    }

    // calculate and populate the cost of the outbound data transfer per-region
    let external_data_transfer_options = result.data.external_data_transfer.as_ref().unwrap();

    let mut cheapest_data_transfer_in_region: HashMap<AwsRegion, f64> = HashMap::new();

    for external_data_transfer in external_data_transfer_options {
        if !external_data_transfer.from_region_code.is_empty()
            && !UNSUPPORTED_REGIONS.contains(&external_data_transfer.from_region_code.as_str())
        {
            let region = AwsRegion::from_str(&external_data_transfer.from_region_code).unwrap();
            let current_price = cheapest_data_transfer_in_region.get(&region);

            if current_price.is_none()
                || *current_price.unwrap() > external_data_transfer.price_per_gb
            {
                cheapest_data_transfer_in_region
                    .insert(region, external_data_transfer.price_per_gb);
            }
        }
    }

    let mut data_transfer_cost: HashMap<AwsRegion, f64> = HashMap::new();

    for (region, price) in cheapest_data_transfer_in_region {
        data_transfer_cost.insert(region, price);
    }

    for region in deployment_configuration.regions {
        let control_plane_price = control_plane_cost.get(&region).unwrap();
        let node_price = node_cost.get(&region).unwrap();
        let storage_price = storage_cost.get(&region).unwrap();
        let data_transfer_price = data_transfer_cost.get(&region).unwrap();

        let control_plane_monthly_price = ((control_plane_price * 730.0) * 100.0).round() / 100.0;
        let control_plane_storage_price = (8.0 * storage_price * 100.0).round() / 100.0;
        let node_monthly_price =
            ((deployment_configuration.node_count * node_price * 730.0) * 100.0).round() / 100.0;
        let node_storage_price = ((deployment_configuration.storage_size_gb
            * storage_price
            * deployment_configuration.node_count)
            * 100.0)
            .round()
            / 100.0;
        let data_transfer_price =
            (data_transfer_price * deployment_configuration.outbound_data_gb as f64 * 100.0)
                .round()
                / 100.0;

        let total_cost = control_plane_monthly_price
            + control_plane_storage_price
            + node_monthly_price
            + node_storage_price
            + data_transfer_price;

        // round to 2 dp
        let total_cost = (total_cost * 100.0).round() / 100.0;

        let mut breakdown = IndexMap::new();

        breakdown.insert(String::from("Control Plane"), control_plane_monthly_price);
        breakdown.insert(
            String::from("Control Plane Storage"),
            control_plane_storage_price,
        );

        breakdown.insert(String::from("Instances"), node_monthly_price);
        breakdown.insert(String::from("Storage"), node_storage_price);
        breakdown.insert(String::from("Data Transfer"), data_transfer_price);
        breakdown.insert(String::from("Total"), total_cost);

        total_deployment_cost.insert(region, breakdown);
    }

    total_deployment_cost
}

pub async fn calculate_cheapest_deployment(
    regions: Vec<AwsRegion>,
) -> HashMap<AwsRegion, IndexMap<String, f64>> {
    calculate_deployment(DeploymentConfiguration {
        regions,
        control_plane_specs: None,
        spot_control_plane: true,
        node_specs: (1, 1.0),
        storage_size_gb: 8.0,
        node_count: 1.0,
        outbound_data_gb: 0,
    })
    .await
}

pub async fn calculate_large_deployment(
    regions: Vec<AwsRegion>,
) -> HashMap<AwsRegion, IndexMap<String, f64>> {
    calculate_deployment(DeploymentConfiguration {
        regions: regions,
        control_plane_specs: Some((2, 4.0)),
        spot_control_plane: false,
        node_specs: (4, 8.0),
        storage_size_gb: 50.0,
        node_count: 3.0,
        outbound_data_gb: 1000,
    })
    .await
}
