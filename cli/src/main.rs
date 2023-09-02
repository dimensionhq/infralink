mod api;
mod commands;
mod constants;
mod core;
mod models;

use crate::core::{executor, parser};
use miette::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Parse the arguments passed in and forward it to the correct command
    let validated_command = parser::parse().await?;

    executor::execute(validated_command).await?;

    Ok(())
}
