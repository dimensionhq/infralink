use crate::db;
use crate::models::on_demand_pricing::{BulkPricingResponse, OnDemandInstance};
use crate::models::spot_pricing::SpotInstance;
use crate::models::storage::Storage;
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::primitives::DateTime;
use aws_sdk_ec2::types::{InstanceType, SpotPrice};
use colored::Colorize;
use futures_util::stream::StreamExt;
use regex::Regex;
use reqwest;
use sqlx::PgPool;
use std::collections::HashMap;

fn convert_to_spot_instance(spot_price: &SpotPrice) -> SpotInstance {
    SpotInstance {
        instance_type: spot_price
            .instance_type
            .as_ref()
            .unwrap()
            .as_str()
            .to_string(),
        spot_price: spot_price
            .spot_price
            .as_ref()
            .unwrap()
            .parse::<f64>()
            .unwrap_or(0.0),
    }
}

// Function to update spot pricing for a specific region
pub async fn update_spot_pricing_for_region(
    pool: PgPool,
    region_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let region_code_string = region_code.to_owned();

    let config = aws_config::from_env()
        .region(Region::new(region_code_string.clone()))
        .load()
        .await;

    let client = aws_sdk_ec2::Client::new(&config);

    let start_time_secs = chrono::Utc::now().timestamp();
    let start_time = DateTime::from_secs(start_time_secs);

    let request = client
        .describe_spot_price_history()
        .instance_types(InstanceType::T3Micro)
        .instance_types(InstanceType::T2Micro)
        .instance_types(InstanceType::T3aMicro)
        .instance_types(InstanceType::T4gMicro)
        .instance_types(InstanceType::T4gSmall)
        .max_results(100)
        .start_time(start_time)
        .product_descriptions("Linux/UNIX");

    let result = request.send().await?;

    let spot_price_history = result
        .spot_price_history
        .ok_or("No spot price history found")?;

    let mut latest_prices: HashMap<String, Vec<SpotInstance>> = HashMap::new();

    for price in spot_price_history.iter() {
        let az = price
            .availability_zone
            .as_ref()
            .ok_or("Availability zone missing")?
            .to_string();

        let instance_price = convert_to_spot_instance(price);

        latest_prices
            .entry(az)
            .or_default()
            .push(instance_price);
    }

    crate::db::insert::spot_pricing_in_bulk(
        &pool,
        region_code_string.clone(),
        latest_prices.clone(),
    )
    .await?;

    println!("Updated spot pricing for {}.", region_code.bright_cyan());

    Ok(())
}

// Function to update pricing for a specific region (on-demand)
pub async fn update_pricing_for_region(
    pool: PgPool,
    region_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the URL for the specific region
    let url = format!(
        "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonEC2/current/{}/index.json",
        region_code
    );

    // Create a client
    let client = reqwest::Client::new();

    // Send a GET request
    let response = client.get(&url).send().await?;

    // Download the content
    let mut content = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        content.extend_from_slice(&chunk);
    }

    // Parse the JSON content
    let region_response: BulkPricingResponse = serde_json::from_slice(&content)?;

    drop(content);

    // Create a map of the sku id -> instance name, vcpu count, and memory
    let mut sku_to_instance: HashMap<String, OnDemandInstance> = HashMap::new();
    // Create a map of the sku id -> storage
    let mut sku_to_storage: HashMap<String, Storage> = HashMap::new();

    if let Some(products) = &region_response.products {
        for (_, details) in products.iter() {
            if let Some(attribute) = &details.attributes {
                if details.product_family == "Storage" {
                    let sku = details.sku.as_str();

                    sku_to_storage.insert(
                        sku.to_owned(),
                        Storage {
                            volume_api_name: attribute.volume_api_name.as_ref().unwrap().clone(),
                            storage_media: if attribute.storage_media.as_ref().unwrap()
                                == "HDD-backed"
                            {
                                "HDD".to_string()
                            } else {
                                "SSD".to_string()
                            },
                            price_per_gb_month: 0.0,
                            region: region_code.to_string(),
                        },
                    );
                }

                if let Some(instance_name) = &attribute.instance_type {
                    let instance_name = instance_name.as_str();

                    let vcpu_count: f32 = attribute.vcpu.as_ref().unwrap().0;

                    // for memory, parse the string and extract the number
                    // 4 GiB -> 4
                    let memory = attribute
                        .memory
                        .as_ref()
                        .unwrap()
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f32>()
                        .unwrap_or(0.0);

                    let storage = attribute.storage.as_ref().unwrap();

                    let physical_processor = attribute.physical_processor.as_ref().unwrap();

                    let arch = if physical_processor.starts_with("AWS Graviton")
                        || physical_processor.starts_with("Ampere")
                    {
                        "arm64".to_string()
                    } else {
                        "x86_64".to_string()
                    };

                    if memory != 0.0 {
                        sku_to_instance.insert(
                            instance_name.to_owned(),
                            OnDemandInstance {
                                region: region_code.to_string(),
                                instance_name: instance_name.to_string(),
                                vcpu_count,
                                memory,
                                price_per_hour: 0.0,
                                arch,
                                storage: storage.to_owned(),
                            },
                        );
                    }
                }
            }
        }
    }

    let pattern = Regex::new(r"(.*) per On Demand Linux ([A-Za-z0-9.-]+) Instance Hour").unwrap();

    if let Some(terms) = &region_response.terms {
        for details in terms.on_demand.values() {
            for term in details.values() {
                for price_dimensions in term.price_dimensions.values() {
                    if sku_to_storage.contains_key(&term.sku) {
                        sku_to_storage
                            .get_mut(&term.sku)
                            .unwrap()
                            .price_per_gb_month =
                            price_dimensions.price_per_unit.usd.as_ref().unwrap().0;
                    } else {
                        let description = &price_dimensions.description.as_ref().unwrap();

                        if !description.is_empty() {
                            let captures = pattern.captures(description);

                            if let Some(captures) = captures {
                                let instance_name = captures.get(2).unwrap().as_str();

                                if let Some(instance) = sku_to_instance.get_mut(instance_name) {
                                    if let Some(price_per_hour) =
                                        price_dimensions.price_per_unit.usd.as_ref()
                                    {
                                        // Convert the price per hour to f64 and round to 5 decimal places
                                        instance.price_per_hour =
                                            (price_per_hour.0 * 100000.0).round() / 100000.0;
                                    } else {
                                        instance.price_per_hour = 0.0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    drop(region_response);

    // Prepare a vector to collect all pricing entries
    let instance_entries: Vec<OnDemandInstance> = sku_to_instance.values().cloned().collect();
    let storage_entries: Vec<Storage> = sku_to_storage.values().cloned().collect();

    drop(sku_to_instance);
    drop(sku_to_storage);

    let instance_entries_len = instance_entries.len();
    let storage_entries_len = storage_entries.len();

    // Insert on demand pricing
    db::insert::on_demand_pricing_in_bulk(&pool, instance_entries).await?;

    // Insert storage pricing
    db::insert::storage_pricing_in_bulk(&pool, storage_entries).await?;

    println!(
        "Updated pricing for {} with {} instance types and {} storage types.",
        region_code.bright_cyan(),
        instance_entries_len.to_string().bright_green(),
        storage_entries_len.to_string().bright_green()
    );

    Ok(())
}
