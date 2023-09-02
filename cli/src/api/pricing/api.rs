use std::{collections::HashMap, str::FromStr};

use miette::{IntoDiagnostic, Result};
use serde_json::json;

use crate::{
    constants::regions::UNSUPPORTED_REGIONS,
    models::{
        instance::AwsInstance, network::BandwidthTier, region::AwsRegion, storage::StoragePricing,
    },
};

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
        if instance.price_per_hour == 0.0 {
            continue;
        }

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

pub async fn get_external_bandwidth_pricing(
    regions: Option<Vec<AwsRegion>>,
    bandwidth: u64,
) -> Result<HashMap<AwsRegion, f64>> {
    // Build a reqwest client that uses rustls
    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();

    let url = format!("{}/{}", API_URL, "pricing/data-transfer/external");

    // Build the request body based on the arguments
    let mut body = json!({});

    if let Some(ref regions) = regions {
        body["regions"] = json!(regions.iter().map(|i| i.code()).collect::<Vec<String>>());
    }

    body["start-range"] = json!(bandwidth);

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .into_diagnostic()
        .unwrap();

    let results = response
        .json::<Vec<BandwidthTier>>()
        .await
        .into_diagnostic()
        .unwrap();

    let mut total_prices: HashMap<AwsRegion, f64> = HashMap::new();

    // Group the results by region
    let mut results_by_region: HashMap<AwsRegion, Vec<BandwidthTier>> = HashMap::new();

    for tier in results {
        if !tier.from_region_code.is_empty()
            && !UNSUPPORTED_REGIONS.contains(&tier.from_region_code.as_str())
        {
            results_by_region
                .entry(AwsRegion::from_str(&tier.from_region_code).unwrap())
                .or_insert_with(Vec::new)
                .push(tier);
        }
    }

    // Calculate the total cost of the bandwidth for each region
    for (region, tiers) in results_by_region {
        let mut total_price = 0.0;
        let mut remaining_bandwidth = bandwidth as f64;

        for tier in tiers {
            let tier_bandwidth = (tier.end_range - tier.start_range) as f64;
            let tier_price = if remaining_bandwidth > tier_bandwidth {
                tier_bandwidth * tier.price_per_gb
            } else {
                remaining_bandwidth * tier.price_per_gb
            };

            total_price += tier_price;
            remaining_bandwidth -= tier_bandwidth;
            if remaining_bandwidth <= 0.0 {
                break;
            }
        }

        // Round the price to 2dp
        total_price = (total_price * 100.0).round() / 100.0;

        total_prices.insert(region, total_price);
    }

    Ok(total_prices)
}

pub async fn get_storage_pricing(
    regions: Option<Vec<AwsRegion>>,
    storage_media: Option<String>,
    volume_api_name: Option<String>,
    storage_amount: f64,
) -> Result<HashMap<AwsRegion, HashMap<String, f64>>> {
    // Build a reqwest client that uses rustls
    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();

    let url = format!("{}/{}", API_URL, "pricing/storage");

    // Build the request body based on the arguments
    let mut body = json!({});

    if let Some(ref regions) = regions {
        body["regions"] = json!(regions.iter().map(|i| i.code()).collect::<Vec<String>>());
    }

    if let Some(ref storage_media) = storage_media {
        body["storage-media"] = json!(storage_media);
    }

    if let Some(ref volume_api_name) = volume_api_name {
        body["volume-api-name"] = json!(volume_api_name);
    }

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .into_diagnostic()
        .unwrap();

    let results = response
        .json::<Vec<StoragePricing>>()
        .await
        .into_diagnostic()
        .unwrap();

    let mut prices: HashMap<AwsRegion, HashMap<String, f64>> = HashMap::new();

    for result in results {
        if !result.region.is_empty() && !UNSUPPORTED_REGIONS.contains(&result.region.as_str()) {
            let region = AwsRegion::from_str(&result.region).unwrap();

            let region_prices = prices.entry(region).or_insert_with(HashMap::new);

            let total_price = result.price_per_gb_month * storage_amount;

            // Round the price to 2dp
            let total_price = (total_price * 100.0).round() / 100.0;

            region_prices.insert(result.volume_api_name, total_price);
        }
    }

    Ok(prices)
}
