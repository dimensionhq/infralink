use std::str::FromStr;

use aws_sdk_account::types::RegionOptStatus;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Cell};
use comfy_table::{Color, Table};
use inquire::{Password, PasswordDisplayMode, Select, Text};
use keyring::Entry;
use miette::Result;

use crate::models::region::{AwsRegion, Region};
use crate::{
    api,
    constants::render_config::RENDER_CONFIG,
    core::config::{InfrastructureConfiguration, InternalAWSConfiguration, InternalConfiguration},
    models::cloud_provider::CloudProvider,
};

pub async fn execute() -> Result<()> {
    println!(
        "{}",
        "👋 Welcome! Let's get started by setting up your infrastructure configuration."
            .bright_green()
    );
    // get the app name - can default to current directory name
    let app_name = Text::new("app name:")
        .with_render_config(*RENDER_CONFIG)
        .with_default(
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .prompt_skippable()
        .unwrap()
        .unwrap();

    // share a quick link to our cloud provider selection guide
    println!(
        "💁 For help with selecting a cloud provider based on factors like cost, availability, and more, see: {}.",
        "https://infralink.io/docs/choosing-a-cloud-provider".bright_magenta()
    );

    // get the cloud provider the user wants to deploy to
    let cloud_provider = CloudProvider::from_str(
        Select::new("cloud provider:", vec!["aws", "azure", "gcp", "oracle"])
            .with_render_config(*RENDER_CONFIG)
            .prompt()
            .unwrap(),
    )
    .unwrap();

    let internal_configuration: InternalConfiguration;

    match cloud_provider {
        CloudProvider::Aws => {
            if InternalAWSConfiguration::exists(app_name.clone()) {
                // load the user's AWS credentials from ~/.infralink/<app_name>
                internal_configuration =
                    InternalConfiguration::AWS(InternalAWSConfiguration::load(app_name.clone()));
            } else {
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
                    .unwrap();

                // store the secret access key id in ~/.infralink/<app_name>
                InternalAWSConfiguration::new(access_key_id.clone()).save(app_name.clone());

                // get the user's secret access key
                let secret_access_key = Password::new("secret access key:")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .with_render_config(*RENDER_CONFIG)
                    .prompt()
                    .unwrap();

                internal_configuration = InternalConfiguration::AWS(InternalAWSConfiguration::new(
                    access_key_id.clone(),
                ));

                let entry = Entry::new("infralink", &access_key_id).unwrap();

                // Store the secret access key in the keyring
                match entry.set_password(&secret_access_key) {
                    Ok(()) => println!("🔐 Credentials securely stored in Vault."),
                    Err(err) => eprintln!("Failed to store credentials: {}", err),
                }
            }
        }
        CloudProvider::Azure => todo!(),
        CloudProvider::Gcp => todo!(),
        CloudProvider::None => todo!(),
    }

    // guide the user to selecting a region
    println!(
        "🌏 Select a region to deploy your app to. For guidance, see: {}.",
        "https://infralink.io/docs/choosing-a-region"
            .bright_magenta()
            .underline()
    );

    let mut region: Region = Region::None;

    // get the regions available for the user's cloud provider & account
    match cloud_provider {
        CloudProvider::Aws => {
            if let InternalConfiguration::AWS(aws_config) = internal_configuration {
                let regions = api::aws::api::list_regions(aws_config).await;

                let mut table = Table::new();

                table.set_header(vec![
                    "name",
                    "code",
                    "small deployment",
                    "large deployment",
                    "status",
                ]);

                let mut regions_vec: Vec<(AwsRegion, RegionOptStatus)> =
                    regions.into_iter().collect();

                regions_vec.sort_by(|a, b| {
                    let (_, _, (small_deployment_a, _)) = a.0.to_string_with_price();
                    let (_, _, (small_deployment_b, _)) = b.0.to_string_with_price();
                    small_deployment_a.partial_cmp(&small_deployment_b).unwrap()
                });

                for (region, status) in &regions_vec {
                    let (display_name, code, (small_deployment, large_deployment)) =
                        region.to_string_with_price();

                    table.add_row(
                        vec![
                            Cell::new(display_name.to_string()).fg(Color::Blue),
                            Cell::new(code.to_string()).fg(Color::Cyan),
                            Cell::new(small_deployment.to_string()).fg(Color::Green),
                            Cell::new(large_deployment.to_string()).fg(Color::Green),
                            match status {
                                RegionOptStatus::Enabled => Cell::new("enabled").fg(Color::Green),
                                RegionOptStatus::EnabledByDefault => {
                                    Cell::new("available").fg(Color::Green)
                                }
                                RegionOptStatus::Disabled => Cell::new("opt-in").fg(Color::Yellow),
                                _ => Cell::new("unavailable").fg(Color::Red),
                            },
                        ]
                        .into_iter(),
                    );
                }

                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS);

                println!("{}", table);

                let region_codes: Vec<String> = regions_vec
                    .iter()
                    .map(|(region, _)| region.code().to_string())
                    .collect();

                let region_codes_str: Vec<&str> = region_codes.iter().map(AsRef::as_ref).collect();

                let aws_region = AwsRegion::from_str(
                    Select::new("region:", region_codes_str)
                        .with_render_config(*RENDER_CONFIG)
                        .prompt()
                        .unwrap(),
                )
                .unwrap();

                region = Region::Aws(aws_region);
            }
        }
        CloudProvider::Azure => todo!(),
        CloudProvider::Gcp => todo!(),
        CloudProvider::None => todo!(),
    }

    InfrastructureConfiguration::builder()
        .with_app_name(app_name)
        .with_cloud_provider(cloud_provider)
        .with_region(region)
        .build()
        .save::<&str>(None);

    Ok(())
}
