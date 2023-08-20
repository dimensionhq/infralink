use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Region {
    Aws(AwsRegion),
    Azure(AzureRegion),
    Gcp(GcpRegion),
    Oracle(OracleRegion),
    None,
}

#[derive(Serialize, Deserialize)]
pub enum AwsRegion {
    USEast1,
    USEast2,
    USWest1,
    USWest2,
}

#[derive(Serialize, Deserialize)]
pub enum AzureRegion {
    EastUS,
    EastUS2,
    WestUS,
    WestUS2,
}

#[derive(Serialize, Deserialize)]
pub enum GcpRegion {
    USEast1,
    USEast4,
    USWest1,
    USWest2,
}

#[derive(Serialize, Deserialize)]
pub enum OracleRegion {
    USEast1,
    USEast4,
    USWest1,
    USWest2,
}

impl FromStr for AwsRegion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us-east-1" => Ok(AwsRegion::USEast1),
            "us-east-2" => Ok(AwsRegion::USEast2),
            "us-west-1" => Ok(AwsRegion::USWest1),
            "us-west-2" => Ok(AwsRegion::USWest2),
            _ => Err(String::from("invalid region")),
        }
    }
}
