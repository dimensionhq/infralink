use std::str::FromStr;

use types::{
    config::InfrastructureConfiguration, deployment_configuration::DeploymentConfiguration,
    region::AwsRegion,
};

pub async fn calculate_cost(configuration: InfrastructureConfiguration) {
    let result = math::calculate_deployment(DeploymentConfiguration {
        regions: vec![AwsRegion::from_str(&configuration.app.region).unwrap()],
        control_plane_specs: Some((1, 1.0)),
        spot_control_plane: false,
        node_specs: (
            configuration.shape.as_ref().unwrap().vcpu,
            configuration.shape.as_ref().unwrap().memory,
        ),
        storage_size_gb: configuration.storage.unwrap().size,
        node_count: 1.0,
        outbound_data_gb: 0,
    })
    .await;

    println!("Cost: ${:#?}", result);
}
