use colored::Colorize;
use miette::Result;

use crate::api;
use crate::core::{prompt, table};
use types::region::Region;

use types::{
    cloud_provider::CloudProvider,
    config::{InfrastructureConfiguration, InternalAWSConfiguration, InternalConfiguration},
};

pub async fn execute() -> Result<()> {
    println!(
        "{}",
        "👋 Welcome! Let's get started by setting up your infrastructure configuration."
            .bright_green()
    );
    // get the app name - can default to current directory name
    let app_name = prompt::app_name()?;

    // share a quick link to our cloud provider selection guide
    println!(
        "💁 For help with selecting a cloud provider based on factors like cost, availability, and more, see: {}.",
        "https://infralink.io/docs/choosing-a-cloud-provider".bright_magenta()
    );

    // get the cloud provider the user wants to deploy to
    let cloud_provider = prompt::cloud_provider()?;

    #[allow(unused_assignments)]
    let mut internal_configuration: InternalConfiguration = InternalConfiguration::None;

    match cloud_provider {
        CloudProvider::Aws => {
            if InternalAWSConfiguration::exists(app_name.clone()) {
                // load the user's AWS credentials from ~/.infralink/<app_name>
                internal_configuration =
                    InternalConfiguration::Aws(InternalAWSConfiguration::load(app_name.clone()));
            } else {
                // guide the user to getting their AWS credentials
                println!(
                    "🔑 For help with getting your AWS credentials, see: {}.",
                    "https://infralink.io/docs/getting-your-cloud-provider-credentials?provider=aws"
                        .bright_magenta()
                        .underline()
                );

                internal_configuration = prompt::cloud_credentials(app_name.clone())?;
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
            if let InternalConfiguration::Aws(aws_config) = internal_configuration {
                let regions = api::aws::api::list_regions(aws_config).await?;

                table::render_region_pricing(regions.clone()).await;

                region = prompt::region(regions)?;
            }
        }
        CloudProvider::Azure => todo!(),
        CloudProvider::Gcp => todo!(),
        CloudProvider::None => panic!("No cloud provider selected."),
    }

    InfrastructureConfiguration::builder()
        .with_app_name(app_name)
        .with_cloud_provider(cloud_provider)
        .with_region(region)
        .build()
        .save::<&str>(None);

    Ok(())
}
