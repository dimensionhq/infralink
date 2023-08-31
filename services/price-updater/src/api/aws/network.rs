use std::collections::HashMap;

use colored::Colorize;
use futures_util::StreamExt;
use sqlx::PgPool;

use crate::db;
use crate::models::network::{DataTransferResponse, InterRegionPrice};

pub async fn update_inter_region_networking_pricing(
    pool: PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the URL for the specific region
    let url = "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AWSDataTransfer/current/index.json".to_string();

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
    let network_response: DataTransferResponse = serde_json::from_slice(&content)?;

    let mut transfer_prices: HashMap<String, InterRegionPrice> = HashMap::new();

    // primary region regex (so we don't capture wavelength regions)
    let primary_region_regex = regex::Regex::new(r"^[a-z]{2}-[a-z]+-[0-9]+$").unwrap();

    for (sku, product) in network_response.products {
        if let Some(attributes) = product.attributes {
            if let Some(transfer_type) = attributes.transfer_type {
                if transfer_type == "InterRegion Outbound" {
                    let from_region_code = attributes.from_region_code.unwrap();
                    let to_region_code = attributes.to_region_code.unwrap();

                    if primary_region_regex.is_match(&from_region_code)
                        && primary_region_regex.is_match(&to_region_code)
                    {
                        transfer_prices.insert(
                            sku,
                            InterRegionPrice {
                                price_per_gb: 0.0,
                                from_region_code,
                                to_region_code,
                            },
                        );
                    }
                }
            }
        }
    }

    for (sku, term_map) in network_response.terms.on_demand {
        for (_, term) in term_map {
            if transfer_prices.contains_key(&sku) {
                for (_, dimension) in term.price_dimensions {
                    if let Some(price) = dimension.price_per_unit.usd {
                        let rounded_price = (price.0 * 1000.0).round() / 1000.0;

                        if let Some(instance) = transfer_prices.get_mut(&sku) {
                            instance.price_per_gb = rounded_price;
                        }
                    }
                }
            }
        }
    }

    drop(content);

    let transfer_prices_len = transfer_prices.len();

    db::insert::inter_region_data_transfer_in_bulk(&pool, transfer_prices).await?;

    println!(
        "Updated inter-region pricing for {} routes.",
        transfer_prices_len.to_string().bright_green()
    );

    Ok(())
}
