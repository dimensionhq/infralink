use std::str::FromStr;

use inquire::{Password, PasswordDisplayMode, Select, Text};
use miette::Result;
use owo_colors::OwoColorize;

use crate::{
    constants::render_config::RENDER_CONFIG, core::config::InfrastructureConfiguration,
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
        "https://infralink.io/docs/choosing-a-cloud-provider".bright_magenta().underline()
    );

    // get the cloud provider the user wants to deploy to
    let cloud_provider = CloudProvider::from_str(
        Select::new("cloud provider:", vec!["aws", "azure", "gcp", "oracle"])
            .with_render_config(*RENDER_CONFIG)
            .prompt()
            .unwrap(),
    )
    .unwrap();

    // share a link to our cloud provider credentials guide
    println!(
        "🔑 For help with getting your cloud provider credentials on {}, see: {}.",
        cloud_provider.to_string().bright_magenta(),
        format!(
            "https://infralink.io/docs/getting-your-cloud-provider-credentials?provider={}",
            cloud_provider.to_string()
        )
        .bright_magenta()
        .underline()
    );

    match cloud_provider {
        CloudProvider::Aws => {
            // get user's cloud provider credentials and securely store them
            let access_key_id = Text::new("access key id:")
                .with_render_config(*RENDER_CONFIG)
                .prompt()
                .unwrap();

            // get the user's secret access key
            let secret_access_key = Password::new("secret access key:")
                .with_display_mode(PasswordDisplayMode::Masked)
                .without_confirmation()
                .with_render_config(*RENDER_CONFIG)
                .prompt()
                .unwrap();

            todo!("store credentials")
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

    InfrastructureConfiguration::builder()
        .with_app_name(app_name)
        .with_cloud_provider(cloud_provider)
        // .with_region()
        .build()
        .save::<&str>(None);

    Ok(())
}
