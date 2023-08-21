use std::str::FromStr;

use aws_config::{AppName, SdkConfig};
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_account::types::RegionOptStatus;
use linked_hash_map::LinkedHashMap;

use crate::{core::config::InternalAWSConfiguration, models::region::AwsRegion};

pub async fn list_regions(
    internal_config: InternalAWSConfiguration,
) -> LinkedHashMap<AwsRegion, RegionOptStatus> {
    let sdk_config = SdkConfig::builder()
        .app_name(AppName::new("infralink").unwrap())
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            internal_config.credentials.access_key_id,
            internal_config.credentials.secret_access_key,
            None,
            None,
            "infralink",
        )))
        .region(aws_types::region::Region::new("us-east-1"))
        .build();

    let client = aws_sdk_account::Client::new(&sdk_config);

    let regions = client
        .list_regions()
        .region_opt_status_contains(RegionOptStatus::EnabledByDefault)
        .region_opt_status_contains(RegionOptStatus::Enabled)
        .region_opt_status_contains(RegionOptStatus::Disabled)
        .max_results(50)
        .send()
        .await
        .unwrap();

    let mut regions_map: LinkedHashMap<AwsRegion, RegionOptStatus> = LinkedHashMap::new();

    for region in regions.regions.unwrap() {
        regions_map.insert(
            AwsRegion::from_str(region.region_name.unwrap().as_str()).unwrap(),
            region.region_opt_status.unwrap(),
        );
    }

    regions_map
}
