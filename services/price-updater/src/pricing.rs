use crate::constants::regions::AWS_REGIONS;
use crate::db;
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::primitives::DateTime;
use aws_sdk_ec2::types::{InstanceType, SpotPrice};
use colored::Colorize;
use futures_util::stream::StreamExt;
use regex::Regex;
use reqwest;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Map, Value};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MySpotPrice {
    pub instance_type: String,
    pub spot_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BulkPricingResponse {
    pub products: Option<HashMap<String, Product>>,
    pub terms: Option<Terms>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Terms {
    #[serde(rename = "OnDemand")]
    pub on_demand: HashMap<String, HashMap<String, OnDemandTerms>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OnDemandTerms {
    pub price_dimensions: HashMap<String, PriceDimension>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceDimension {
    pub description: Option<String>,
    pub price_per_unit: PricePerUnit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PricePerUnit {
    #[serde(rename = "USD")]
    pub usd: Option<ForceF32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub sku: String,
    pub attributes: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub instance_type: Option<String>,
    pub vcpu: Option<ForceF32>,
    pub memory: Option<String>,
    pub storage: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Pricing {
    pub region: String,
    pub instance_name: String,
    pub vcpu_count: f64,
    pub memory: f64,
    pub price_per_hour: f64,
    pub storage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForceF32(#[serde(deserialize_with = "str_to_f32")] f32);

fn str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(deserializer).and_then(|s| s.parse().map_err(D::Error::custom))
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
    let mut sku_to_instance: Map<String, Value> = Map::new();

    if let Some(products) = region_response.products {
        for (_, details) in products {
            if let Some(attributes) = details.attributes {
                if let Some(instance_name) = attributes.instance_type {
                    let instance_name = instance_name.as_str();

                    let vcpu_count: f32 = attributes.vcpu.unwrap().0;

                    // for memory, parse the string and extract the number
                    // 4 GiB -> 4

                    // Check if memory isn't NA

                    let memory = attributes
                        .memory
                        .unwrap()
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f32>()
                        .unwrap_or(0.0);

                    let storage = attributes.storage.unwrap();

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

    if let Some(terms) = region_response.terms {
        for (_, details) in terms.on_demand {
            for (_, term) in details {
                for (_, price_dimensions) in term.price_dimensions {
                    let description = &price_dimensions.description.unwrap();

                    if !description.is_empty() {
                        let captures = pattern.captures(description);

                        if let Some(captures) = captures {
                            let instance_name = captures.get(2).unwrap().as_str();

                            if let Some(instance) = sku_to_instance.get_mut(instance_name) {
                                if let Some(price_per_hour) = price_dimensions.price_per_unit.usd {
                                    // Convert the price per hour to f64 and round to 5 decimal places
                                    instance["price_per_hour"] =
                                        json!((price_per_hour.0 * 100000.0).round() / 100000.0);
                                } else {
                                    instance["price_per_hour"] = json!(0.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // drop(region_response);

    // Prepare a vector to collect all pricing entries
    let mut pricing_entries: Vec<Pricing> = Vec::new();

    // Process the SKU to instance mapping
    for (instance_name, instance_details) in sku_to_instance.iter() {
        // Get the necessary details
        let vcpu_count = instance_details["vcpu_count"].as_f64().unwrap_or(0.0);
        let memory = instance_details["memory"].as_f64().unwrap_or(0.0);
        let price_per_hour = instance_details["price_per_hour"].as_f64().unwrap_or(0.0);
        let storage = instance_details["storage"].as_str().unwrap_or("");

        if price_per_hour != 0.0 && memory != 0.0 {
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

    drop(sku_to_instance);

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
