use std::collections::HashMap;

use anyhow::Result;
use sqlx::Error as SqlxError;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;

use crate::pricing::{MySpotPrice, Pricing};

pub async fn connect() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(PgPool::connect(&database_url).await?)
}

const MAX_RETRIES: usize = 5;

pub async fn insert_on_demand_pricing_in_bulk(
    pool: &PgPool,
    entries: Vec<Pricing>,
) -> Result<(), SqlxError> {
    let mut retries = 0;

    loop {
        // Start a transaction
        let mut tx = pool.begin().await?;

        let values: Vec<String> = entries
            .iter()
            .map(|entry| {
                format!(
                    "('{}', '{}', {}, {}, {}, '{}', NOW())",
                    entry.region,
                    entry.instance_name,
                    entry.vcpu_count,
                    entry.memory,
                    entry.price_per_hour,
                    entry.storage
                )
            })
            .collect();

        let insert_query = format!(
            "INSERT INTO on_demand (region, instance_type, vcpu_count, memory, price_per_hour, storage, updated_at)
            VALUES {}
            ON CONFLICT (region, instance_type)
            DO UPDATE SET vcpu_count = excluded.vcpu_count, memory = excluded.memory, price_per_hour = excluded.price_per_hour, storage = excluded.storage, updated_at = NOW()",
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
                sleep(Duration::from_millis(200)).await;
            }
        }
    }
}

pub async fn insert_spot_pricing_in_bulk(
    pool: &PgPool,
    region: String,
    prices: HashMap<String, Vec<MySpotPrice>>,
) -> Result<()> {
    // Start a transaction
    let mut tx = pool.begin().await?;

    // Prepare an INSERT statement with ON CONFLICT clause
    let insert_query = "
    INSERT INTO spot (region, availability_zone, instance_type, price_per_hour, updated_at)
    VALUES ($1, $2, $3, $4, NOW())
    ON CONFLICT (region, availability_zone, instance_type)
    DO UPDATE SET price_per_hour = excluded.price_per_hour, updated_at = NOW()";

    // Iterate over the data and execute the queries
    for (availability_zone, spot_prices) in prices.iter() {
        for spot_price in spot_prices.iter() {
            sqlx::query(&insert_query)
                .bind(&region)
                .bind(availability_zone)
                .bind(&spot_price.instance_type)
                .bind(&spot_price.spot_price)
                .execute(&mut *tx)
                .await?;
        }
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}
