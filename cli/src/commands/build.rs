use miette::Result;

use crate::core::config::InternalAWSConfiguration;

pub async fn execute() -> Result<()> {
    let internal_configuration = InternalAWSConfiguration::load("cli".to_string());

    println!("{}", internal_configuration.credentials.secret_access_key);

    Ok(())
}
