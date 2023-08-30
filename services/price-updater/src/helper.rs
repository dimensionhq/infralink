use futures_util::StreamExt;
use sqlx::PgPool;

use crate::{
    api::aws::instance::{update_pricing_for_region, update_spot_pricing_for_region},
    constants::regions::AWS_REGIONS,
};

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
