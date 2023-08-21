mod api;
mod commands;
mod constants;
mod core;
mod models;

use crate::core::parser;
use miette::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Parse the arguments passed in and forward it to the correct command
    parser::parse().await?;

    Ok(())
}
