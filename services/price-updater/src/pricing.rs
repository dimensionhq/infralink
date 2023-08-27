use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::primitives::DateTime;
use aws_sdk_ec2::types::{InstanceType, SpotPrice};
use colored::Colorize;
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::constants::regions::AWS_REGIONS;
use crate::db;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MySpotPrice {
    pub instance_type: String,
    pub spot_price: String,
}

pub struct Pricing {
    pub region: String,
    pub instance_name: String,
    pub vcpu_count: f64,
    pub memory: f64,
    pub price_per_hour: f64,
    pub storage: String,
}

pub async fn update_on_demand_pricing_index(
    pool: PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let regions_stream = futures_util::stream::iter(
        AWS_REGIONS
            .iter()
            .map(|&region| update_pricing_for_region(pool.clone(), region)),
    );

    regions_stream
        .for_each_concurrent(6, |fut| async {
            if let Err(err) = fut.await {
                // Handle the error here, maybe log it
                println!("Failed to update pricing for region: {:?}", err);
            }
        })
        .await;

    Ok(())
}

// Function to update spot pricing index for all regions
pub async fn update_spot_pricing_index(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let regions_stream = futures_util::stream::iter(
        AWS_REGIONS
            .iter()
            .map(|&region| update_spot_pricing_for_region(pool.clone(), region)),
    );

    regions_stream
        .for_each_concurrent(10, |fut| async {
            if let Err(err) = fut.await {
                // Handle the error here, maybe log it
                println!("Failed to update spot pricing for region: {:?}", err);
            }
        })
        .await;

    Ok(())
}

// Function to convert the given SpotPrice to MySpotPrice
fn convert_to_my_spot_price(spot_price: &SpotPrice) -> MySpotPrice {
    MySpotPrice {
        instance_type: spot_price
            .instance_type
            .as_ref()
            .unwrap()
            .as_str()
            .to_string(),
        spot_price: spot_price.spot_price.as_ref().unwrap().clone(),
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

    let mut latest_prices: HashMap<String, Vec<MySpotPrice>> = HashMap::new();

    for price in spot_price_history.iter() {
        let az = price
            .availability_zone
            .as_ref()
            .ok_or("Availability zone missing")?
            .to_string();

        let instance_price = convert_to_my_spot_price(price);

        latest_prices
            .entry(az)
            .or_insert_with(Vec::new)
            .push(instance_price);
    }

    db::insert_spot_pricing_in_bulk(&pool, region_code_string, latest_prices).await?;

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

    // Get the content length for the progress bar
    let content_length = response.content_length().unwrap_or(0);

    // Create a progress bar
    let pb = ProgressBar::new(content_length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Download the content with progress
    let mut content = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        pb.inc(chunk.len() as u64);
        content.extend_from_slice(&chunk);
    }

    pb.finish_with_message(format!(
        "Successfully downloaded EC2 pricing for {}.",
        region_code
    ));

    // Parse the JSON content
    let region_response: Value = serde_json::from_slice(&content)?;

    // Create a map of the sku id -> instance name, vcpu count, and memory
    let mut sku_to_instance: Map<String, Value> = Map::new();

    if let Some(products) = region_response["products"].as_object() {
        for (_, details) in products {
            if let Some(attributes) = details["attributes"].as_object() {
                if let Some(instance_name) = attributes.get("instanceType") {
                    let instance_name = instance_name.as_str().unwrap();

                    let vcpu_count: f32 = attributes["vcpu"]
                        .as_str()
                        .unwrap_or("")
                        .parse()
                        .unwrap_or(0.0);

                    let memory: f32 = attributes["memory"]
                        .as_str()
                        .unwrap_or("")
                        .split_whitespace()
                        .next()
                        .unwrap_or("0.0")
                        .parse()
                        .unwrap_or(0.0);

                    let storage = attributes["storage"].as_str().unwrap();

                    sku_to_instance.insert(
                        instance_name.to_owned(),
                        json!({
                            "instance_name": instance_name,
                            "vcpu_count": vcpu_count,
                            "memory": memory,
                            "storage": storage
                        }),
                    );
                }
            }
        }
    }

    let pattern = Regex::new(r"(.*) per On Demand Linux ([A-Za-z0-9.]+) Instance Hour").unwrap();

    if let Some(terms) = region_response["terms"]["OnDemand"].as_object() {
        for (_, details) in terms {
            for (_, term) in details.as_object().unwrap() {
                for (_, price_dimensions) in term["priceDimensions"].as_object().unwrap() {
                    let description = price_dimensions["description"].as_str().unwrap_or("");

                    if description != "" {
                        let captures = pattern.captures(description);

                        if let Some(captures) = captures {
                            let instance_name = captures.get(2).unwrap().as_str();

                            if let Some(instance) = sku_to_instance.get_mut(instance_name) {
                                // Get the price per hour as a string
                                let price_per_hour_str = price_dimensions["pricePerUnit"]["USD"]
                                    .as_str()
                                    .unwrap_or("0.0");

                                // Convert the price per hour to f64 and round to 5 decimal places
                                if let Ok(price_per_hour) = price_per_hour_str.parse::<f64>() {
                                    instance["price_per_hour"] =
                                        json!((price_per_hour * 100000.0).round() / 100000.0);
                                } else {
                                    println!(
                                        "Failed to parse price for instance: {}",
                                        instance_name
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Prepare a vector to collect all pricing entries
    let mut pricing_entries: Vec<Pricing> = Vec::new();

    // Process the SKU to instance mapping
    for (instance_name, instance_details) in sku_to_instance.iter() {
        // Get the necessary details
        let vcpu_count = instance_details["vcpu_count"].as_f64().unwrap_or(0.0);
        let memory = instance_details["memory"].as_f64().unwrap_or(0.0);
        let price_per_hour = instance_details["price_per_hour"].as_f64().unwrap_or(0.0);
        let storage = instance_details["storage"].as_str().unwrap_or("");

        if price_per_hour != 0.0 {
            // Create an entry for the database
            let pricing_entry = Pricing {
                region: region_code.to_string(),
                instance_name: instance_name.to_string(),
                vcpu_count,
                memory,
                price_per_hour,
                storage: storage.to_string(),
            };

            // Add the entry to the collection
            pricing_entries.push(pricing_entry);
        }
    }

    let pricing_entries_len = pricing_entries.len();

    // Insert all the entries in the database in a single query
    db::insert_on_demand_pricing_in_bulk(&pool, pricing_entries).await?;

    println!(
        "Updated pricing for {} with {} instance types.",
        region_code.bright_cyan(),
        pricing_entries_len.to_string().bright_green()
    );

    Ok(())
}
