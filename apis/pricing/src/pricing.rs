use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest;
use serde_json::{json, Map, Value};
use std::fs::File;
use std::io::Write;

pub async fn get_pricing_for_region(region_code: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);

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

    pb.finish_with_message("Download complete");

    // Parse the JSON content
    let region_response: Value = serde_json::from_slice(&content)?;

    // Create a map of the sku id -> instance name, vcpu count, and memory
    let mut sku_to_instance: Map<String, Value> = Map::new();

    if let Some(products) = region_response["products"].as_object() {
        for (_, details) in products {
            if let Some(attributes) = details["attributes"].as_object() {
                let instance_name = attributes
                    .get("instanceType")
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let vcpu_count_str = attributes.get("vcpu").and_then(Value::as_str).unwrap_or("");
                let vcpu_count: f32 = vcpu_count_str.parse().unwrap_or(0.0);

                let memory_str = attributes
                    .get("memory")
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let memory: f32 = memory_str
                    .split_whitespace()
                    .next()
                    .unwrap_or("0.0")
                    .parse()
                    .unwrap_or(0.0);

                sku_to_instance.insert(
                    instance_name.to_owned(),
                    json!({
                        "instance_name": instance_name,
                        "vcpu_count": vcpu_count,
                        "memory": memory
                    }),
                );
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

                                // Convert the price per hour to f64
                                if let Ok(mut price_per_hour) = price_per_hour_str.parse::<f64>() {
                                    // Round the price per hour to 5 decimal places
                                    price_per_hour = (price_per_hour * 100000.0).round() / 100000.0;

                                    instance["price_per_hour"] = json!(price_per_hour);
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

    // Save the transformed data to a file
    let file_path = format!("regions/pricing/{}.json", region_code);
    let mut file = File::create(file_path)?;
    file.write_all(serde_json::to_string_pretty(&sku_to_instance)?.as_bytes())?;

    println!("File saved successfully!");

    Ok(())
}
