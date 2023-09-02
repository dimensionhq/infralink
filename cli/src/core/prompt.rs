use std::str::FromStr;

use aws_sdk_account::types::RegionOptStatus;
use colored::Colorize;
use inquire::{Password, PasswordDisplayMode, Select, Text};
use keyring::Entry;
use linked_hash_map::LinkedHashMap;
use miette::{IntoDiagnostic, Result};

use crate::{
    constants::render_config::RENDER_CONFIG,
    core::config::{InternalAWSConfiguration, InternalConfiguration},
    models::{
        cloud_provider::CloudProvider,
        region::{AwsRegion, Region},
    },
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
    // guide the user to getting their AWS credentials
    println!(
        "🔑 For help with getting your AWS credentials, see: {}.",
        "https://infralink.io/docs/getting-your-cloud-provider-credentials?provider={}"
            .bright_magenta()
            .underline()
    );

    // get user's cloud provider credentials and securely store them
    let access_key_id = Text::new("access key id:")
        .with_render_config(*RENDER_CONFIG)
        .prompt()
        .into_diagnostic()
        .unwrap();

    // store the secret access key id in ~/.infralink/<app_name>
    InternalAWSConfiguration::new(access_key_id.clone()).save(app_name.clone());

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
    let region_codes: Vec<String> = regions.iter().map(|(region, _)| region.code()).collect();

    let region_codes_str: Vec<&str> = region_codes.iter().map(AsRef::as_ref).collect();

    let aws_region = AwsRegion::from_str(
        Select::new("region:", region_codes_str)
            .with_render_config(*RENDER_CONFIG)
            .prompt()
            .unwrap(),
    )
    .unwrap();

    Ok(Region::Aws(aws_region))
}
