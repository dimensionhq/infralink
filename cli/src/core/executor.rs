use crate::commands;

use super::{error::CommandError, validator::ValidatedOptions};
use miette::Result;

// Execute the command passed in
pub async fn execute(options: ValidatedOptions) -> Result<()> {
    match options {
        ValidatedOptions::Init {} => commands::init::execute().await,
        ValidatedOptions::Help {} => commands::help::execute().await,
        ValidatedOptions::None => {
            // Return the custom error instead of exiting
            return Err(CommandError::CommandNotFound.into());
        }
    }
}
