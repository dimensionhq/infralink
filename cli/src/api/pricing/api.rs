use std::collections::HashMap;

use miette::{IntoDiagnostic, Result};
use serde_json::json;

use crate::models::{instance::AwsInstance, region::AwsRegion};

const API_URL: &str = "https://pricing.infralink.io";

pub async fn get_cheapest_spot_instances(
    regions: Option<Vec<AwsRegion>>,
) -> Result<HashMap<AwsRegion, AwsInstance>> {
    // Build a reqwest client that uses rustls
    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();

    let url = format!("{}/{}", API_URL, "pricing/spot");

    // Build the request body based on the arguments
    let mut body = json!({
        "sort-by": "price_per_hour",
        "sort-order": "asc",
    });

    if let Some(regions) = regions {
        body["regions"] = json!(regions.iter().map(|i| i.code()).collect::<Vec<String>>());
    }

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .into_diagnostic()
        .unwrap();

    let results = response
        .json::<Vec<AwsInstance>>()
        .await
        .into_diagnostic()
        .unwrap();

    let mut cheapest_instances: HashMap<AwsRegion, AwsInstance> = HashMap::new();

    for instance in results {
        if let Some(existing_instance) = cheapest_instances.get(&instance.region) {
            if instance.price_per_hour < existing_instance.price_per_hour {
                cheapest_instances.insert(instance.region.clone(), instance);
            }
        } else {
            cheapest_instances.insert(instance.region.clone(), instance);
        }
    }

    Ok(cheapest_instances)
}

pub async fn get_cheapest_on_demand_instances(
    regions: Option<Vec<AwsRegion>>,
    min_vcpu: Option<f64>,
    min_memory: Option<f64>,
) -> Result<HashMap<AwsRegion, AwsInstance>> {
    // Build a reqwest client that uses rustls
    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();

    let url = format!("{}/{}", API_URL, "pricing/on-demand");

    // Build the request body based on the arguments
    let mut body = json!({
        "sort-by": "price_per_hour",
        "sort-order": "asc",
    });

    if let Some(regions) = regions {
        body["regions"] = json!(regions.iter().map(|i| i.code()).collect::<Vec<String>>());
    }

    if let Some(min_vcpu) = min_vcpu {
        body["min-vcpu"] = json!(min_vcpu);
    }

    if let Some(min_memory) = min_memory {
        body["min-memory"] = json!(min_memory);
    }

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .into_diagnostic()
        .unwrap();

    let results = response
        .json::<Vec<AwsInstance>>()
        .await
        .into_diagnostic()
        .unwrap();

    let mut cheapest_instances: HashMap<AwsRegion, AwsInstance> = HashMap::new();

    for instance in results {
        if let Some(existing_instance) = cheapest_instances.get(&instance.region) {
            if instance.price_per_hour < existing_instance.price_per_hour {
                cheapest_instances.insert(instance.region.clone(), instance);
            }
        } else {
            cheapest_instances.insert(instance.region.clone(), instance);
        }
    }

    Ok(cheapest_instances)
}
