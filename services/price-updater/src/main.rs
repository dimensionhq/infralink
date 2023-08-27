pub mod constants;
pub mod db;
pub mod helper;
pub mod pricing;

use pricing::{update_on_demand_pricing_index, update_spot_pricing_index};
use std::time::Duration;
use tokio::{join, time::sleep};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a new connection pool
    let pool = db::connect().await.unwrap();

    let pool_for_on_demand = pool.clone();
    let pool_for_spot = pool.clone();

    // Task for update_on_demand_pricing_index every 8 hours
    let on_demand_pricing_task = tokio::spawn(async move {
        let interval = Duration::from_secs(8 * 60 * 60);
        loop {
            if let Err(err) = update_on_demand_pricing_index(pool_for_on_demand.clone()).await {
                println!("Failed to update on-demand pricing: {:?}", err);
            }
            sleep(interval).await;
        }
    });

    // Task for update_spot_pricing_index every 2 minutes
    let spot_pricing_task = tokio::spawn(async move {
        let interval = Duration::from_secs(2 * 60);
        loop {
            if let Err(err) = update_spot_pricing_index(pool_for_spot.clone()).await {
                println!("Failed to update spot pricing: {:?}", err);
            }
            sleep(interval).await;
        }
    });

    // Join all tasks to ensure they continue running
    let _ = join!(on_demand_pricing_task, spot_pricing_task);

    Ok(())
}
