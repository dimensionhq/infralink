use std::str::FromStr;

use aws_sdk_account::types::RegionOptStatus;
use colored::Colorize;
use inquire::{Confirm, Password, PasswordDisplayMode, Select, Text};
use keyring::Entry;
use linked_hash_map::LinkedHashMap;
use miette::{IntoDiagnostic, Result};

use constants::render_config::RENDER_CONFIG;

use types::config::{InternalAWSConfiguration, InternalConfiguration};
use types::{
    cloud_provider::CloudProvider,
    region::{AwsRegion, Region},
};

pub fn app_name() -> Result<String> {
    let app_name = Text::new("app name:")
        .with_render_config(*RENDER_CONFIG)
        .with_default(
            std::env::current_dir()
                .into_diagnostic()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .prompt_skippable()
        .into_diagnostic()?
        .unwrap();

    Ok(app_name)
}

pub fn cloud_provider() -> Result<CloudProvider> {
    let cloud_provider = CloudProvider::from_str(
        Select::new("cloud provider:", vec!["aws", "azure", "gcp", "oracle"])
            .with_render_config(*RENDER_CONFIG)
            .prompt()
            .unwrap(),
    )
    .unwrap();

    Ok(cloud_provider)
}

pub fn cloud_credentials(app_name: String) -> Result<InternalConfiguration> {
    // get user's cloud provider credentials and securely store them
    let access_key_id = Text::new("access key id:")
        .with_render_config(*RENDER_CONFIG)
        .prompt()
        .into_diagnostic()
        .unwrap();

    // store the secret access key id in ~/.infralink/<app_name>
    InternalAWSConfiguration::new(access_key_id.clone()).save(app_name);

    // get the user's secret access key
    let secret_access_key = Password::new("secret access key:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .with_render_config(*RENDER_CONFIG)
        .prompt()
        .into_diagnostic()
        .unwrap();

    let internal_configuration =
        InternalConfiguration::Aws(InternalAWSConfiguration::new(access_key_id.clone()));

    let entry = Entry::new("infralink", &access_key_id)
        .into_diagnostic()
        .unwrap();

    // Store the secret access key in the keyring
    match entry.set_password(&secret_access_key) {
        Ok(()) => println!("🔐 Credentials securely stored in Vault."),
        Err(err) => eprintln!("Failed to store credentials: {}", err),
    }

    Ok(internal_configuration)
}

pub fn region(regions: LinkedHashMap<AwsRegion, RegionOptStatus>) -> Result<Region> {
    let region_codes: Vec<String> = regions
        .iter()
        .map(|(region, _)| region.display_name())
        .collect();

    let region_codes_str: Vec<&str> = region_codes.iter().map(AsRef::as_ref).collect();

    let aws_region = AwsRegion::from_display_name(
        Select::new("region:", region_codes_str)
            .with_render_config(*RENDER_CONFIG)
            .prompt()
            .unwrap(),
    )
    .unwrap();

    Ok(Region::Aws(aws_region))
}

pub fn remain_not_logged_in() -> Result<bool> {
    let benefits = format!(
        r#"You're not currently logged in to Infralink Cloud. Infralink Cloud comes with the following benefits:

- 🖥️  Dashboard
- 🔗 Automatic deploys from GitHub
- 📊 Usage analytics 
- 🔍 Cost estimations
- 📝 Detailed logging and monitoring
- 💵 Predictive analytics with up-to 200% savings
- 🚀 Up-to 10x faster deployments with more powerful machines and collaborative caching
- 🙋 Support from K8s experts
- 🛡️  Advanced security compliance

Get started for free with {} {}.
"#,
        "infra".bright_cyan(),
        "login".bright_blue()
    );

    println!("{benefits}");

    // Ask the user if they want to login
    let login = Confirm::new("Would you like to login?")
        .with_render_config(*RENDER_CONFIG)
        .prompt()
        .unwrap();

    Ok(login)
}
