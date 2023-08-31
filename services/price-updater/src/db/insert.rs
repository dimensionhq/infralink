use anyhow::Result;
use std::{collections::HashMap, time::Duration};

use sqlx::Error as SqlxError;
use sqlx::PgPool;

use crate::models::network::InterRegionPrice;
use crate::models::on_demand_pricing::OnDemandInstance;
use crate::models::spot_pricing::SpotInstance;
use crate::models::storage::Storage;

const MAX_RETRIES: usize = 5;

pub async fn on_demand_pricing_in_bulk(
    pool: &PgPool,
    instances: Vec<OnDemandInstance>,
) -> Result<(), SqlxError> {
    let mut retries = 0;

    loop {
        // Start a transaction
        let mut tx = pool.begin().await?;

        let values: Vec<String> = instances
            .iter()
            .map(|entry| {
                format!(
                    "('{}', '{}', {}, {}, {}, '{}', '{}', NOW())",
                    entry.region,
                    entry.instance_name,
                    entry.vcpu_count,
                    entry.memory,
                    entry.price_per_hour,
                    entry.arch,
                    entry.storage
                )
            })
            .collect();

        let insert_query = format!(
            "INSERT INTO on_demand (region, instance_type, vcpu_count, memory, price_per_hour, architecture, storage, updated_at)
            VALUES {}
            ON CONFLICT (region, instance_type)
            DO UPDATE SET vcpu_count = excluded.vcpu_count, memory = excluded.memory, price_per_hour = excluded.price_per_hour, architecture = excluded.architecture, storage = excluded.storage, updated_at = NOW()",
            values.join(", ")
        );

        // Try executing the query
        match sqlx::query(&insert_query).execute(&mut *tx).await {
            Ok(_) => {
                // Commit the transaction if the query is successful
                tx.commit().await?;
                return Ok(());
            }
            Err(e) => {
                // Roll back the transaction in case of an error
                tx.rollback().await?;
                eprintln!("Error occurred: {:?}", e);
                retries += 1;

                if retries >= MAX_RETRIES {
                    return Err(e);
                }

                // Wait for a bit before retrying
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    }
}

pub async fn spot_pricing_in_bulk(
    pool: &PgPool,
    region: String,
    instances: HashMap<String, Vec<SpotInstance>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start a transaction
    let mut tx = pool.begin().await?;

    // Initialize the values string
    let mut values_str = String::new();

    // Iterate over the data to construct the multi-row VALUES part of the SQL statement
    for (availability_zone, spot_prices) in instances.iter() {
        for spot_price in spot_prices.iter() {
            values_str.push_str(&format!(
                "('{}', '{}', '{}', {}, NOW()),",
                region, availability_zone, spot_price.instance_type, spot_price.spot_price
            ));
        }
    }

    // Remove the trailing comma
    values_str.pop();

    // Create the entire SQL query
    let insert_query = format!(
        "INSERT INTO spot (region, availability_zone, instance_type, price_per_hour, updated_at)
        VALUES {}
        ON CONFLICT (region, availability_zone, instance_type)
        DO UPDATE SET price_per_hour = excluded.price_per_hour, updated_at = NOW()",
        values_str
    );

    // Execute the query
    sqlx::query(&insert_query).execute(&mut *tx).await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}

pub async fn inter_region_data_transfer_in_bulk(
    pool: &PgPool,
    transfer_prices: HashMap<String, InterRegionPrice>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    let mut values_str = String::new();

    for transfer_price in transfer_prices.values() {
        let from_region = &transfer_price.from_region_code;
        let to_region = &transfer_price.to_region_code;
        let price = transfer_price.price_per_gb;

        values_str.push_str(&format!(
            "('{}', '{}', {}, NOW()),",
            from_region, to_region, price
        ));
    }

    // Remove the trailing comma
    values_str.pop();

    let insert_query = format!("
        INSERT INTO inter_region_data_transfer (from_region_code, to_region_code, price_per_gb, updated_at)
        VALUES {}
        ON CONFLICT (from_region_code, to_region_code)
        DO UPDATE SET updated_at = NOW()
    ", values_str);

    sqlx::query(&insert_query).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn storage_pricing_in_bulk(
    pool: &PgPool,
    storage_prices: Vec<Storage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    let mut values_str = String::new();

    for storage in storage_prices.iter() {
        let region = &storage.region;
        let storage_media = &storage.storage_media;
        let volume_api_name = &storage.volume_api_name;
        let price = storage.price_per_gb_month;

        values_str.push_str(&format!(
            "('{}', '{}', '{}', {}, NOW()),",
            region, volume_api_name, storage_media, price
        ));
    }

    // Remove the trailing comma
    if !values_str.is_empty() {
        values_str.pop();
    }

    let insert_query = format!(
        "
        INSERT INTO storage (region, volume_api_name, storage_media, price_per_gb_month, updated_at)
        VALUES {}
        ON CONFLICT (region, volume_api_name)
        DO UPDATE SET price_per_gb_month = EXCLUDED.price_per_gb_month, updated_at = NOW()",
        values_str
    );

    sqlx::query(&insert_query).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}
