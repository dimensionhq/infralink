use std::collections::HashMap;

use colored::Colorize;
use futures_util::StreamExt;
use sqlx::PgPool;

use crate::db;
use crate::models::network::{DataTransferResponse, ExternalPrice, ExternalTier, InterRegionPrice};

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

    let mut inter_region_transfer_prices: HashMap<String, InterRegionPrice> = HashMap::new();
    let mut external_transfer_prices: HashMap<String, ExternalPrice> = HashMap::new();

    for (sku, product) in network_response.products {
        if let Some(attributes) = product.attributes {
            if let Some(transfer_type) = attributes.transfer_type {
                if transfer_type == "InterRegion Outbound" {
                    let from_region_code = attributes.from_region_code.unwrap();
                    let to_region_code = attributes.to_region_code.unwrap();

                    if attributes.from_location_type == Some("AWS Region".to_string())
                        && attributes.to_location_type == Some("AWS Region".to_string())
                    {
                        inter_region_transfer_prices.insert(
                            sku,
                            InterRegionPrice {
                                price_per_gb: 0.0,
                                from_region_code,
                                to_region_code,
                            },
                        );
                    }
                } else if transfer_type == "AWS Outbound" && attributes.from_location_type == Some("AWS Region".to_string()) && attributes.to_location == Some("External".to_string()) {
                    external_transfer_prices.insert(
                        sku,
                        ExternalPrice {
                            from_region_code: attributes.from_region_code.unwrap(),
                            tiers: vec![],
                        },
                    );
                }
            }
        }
    }

    for (sku, term_map) in network_response.terms.on_demand {
        for (_, term) in term_map {
            if inter_region_transfer_prices.contains_key(&sku) {
                for (_, dimension) in term.price_dimensions {
                    if let Some(price) = dimension.price_per_unit.usd {
                        let rounded_price = (price.0 * 1000.0).round() / 1000.0;

                        if let Some(inter_region_price) = inter_region_transfer_prices.get_mut(&sku)
                        {
                            inter_region_price.price_per_gb = rounded_price;
                        }
                    }
                }
            } else if external_transfer_prices.contains_key(&sku) {
                for (_, dimension) in term.price_dimensions {
                    if let Some(price) = dimension.price_per_unit.usd {
                        let rounded_price = (price.0 * 1000.0).round() / 1000.0;

                        if let Some(external_transfer_price) =
                            external_transfer_prices.get_mut(&sku)
                        {
                            external_transfer_price.tiers.push(ExternalTier {
                                price_per_gb: rounded_price,
                                start_range: dimension.begin_range.unwrap().0,
                                end_range: dimension.end_range.unwrap().0,
                            });
                        }
                    }
                }

                if let Some(external_transfer_price) = external_transfer_prices.get_mut(&sku) {
                    external_transfer_price.tiers.sort_by(|a, b| {
                        a.start_range
                            .cmp(&b.start_range)
                            .then_with(|| a.end_range.cmp(&b.end_range))
                    });
                }
            }
        }
    }

    drop(content);

    let inter_region_transfer_prices_len = inter_region_transfer_prices.len();
    let external_transfer_prices_len = external_transfer_prices.len();

    db::insert::inter_region_data_transfer_in_bulk(&pool, inter_region_transfer_prices).await?;
    db::insert::external_data_transfer_in_bulk(&pool, external_transfer_prices).await?;

    println!(
        "Updated inter-region pricing for {} routes.",
        inter_region_transfer_prices_len.to_string().bright_green()
    );

    println!(
        "Updated external pricing for {} routes.",
        external_transfer_prices_len.to_string().bright_green()
    );

    Ok(())
}
