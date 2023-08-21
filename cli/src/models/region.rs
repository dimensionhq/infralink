use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Aws(AwsRegion),
    None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum AwsRegion {
    USEast1,
    USEast2,
    USWest1,
    USWest2,
    AfSouth1,
    ApSouth1,
    ApSouth2,
    ApEast1,
    ApSouthEast1,
    ApSouthEast2,
    ApSouthEast3,
    ApSouthEast4,
    ApNorthEast1,
    ApNorthEast2,
    ApNorthEast3,
    CnNorth1,
    CnNorthWest1,
    CaCentral1,
    EuCentral1,
    EuCentral2,
    EuWest1,
    EuWest2,
    EuWest3,
    EuSouth1,
    EuSouth2,
    EuNorth1,
    IlCentral1,
    MeSouth1,
    MeCentral1,
    SaEast1,
    UsGovEast1,
    UsGovWest1,
}

impl FromStr for AwsRegion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us-east-1" => Ok(AwsRegion::USEast1),
            "us-east-2" => Ok(AwsRegion::USEast2),
            "us-west-1" => Ok(AwsRegion::USWest1),
            "us-west-2" => Ok(AwsRegion::USWest2),
            "af-south-1" => Ok(AwsRegion::AfSouth1),
            "ap-south-1" => Ok(AwsRegion::ApSouth1),
            "ap-south-2" => Ok(AwsRegion::ApSouth2),
            "ap-east-1" => Ok(AwsRegion::ApEast1),
            "ap-southeast-1" => Ok(AwsRegion::ApSouthEast1),
            "ap-southeast-2" => Ok(AwsRegion::ApSouthEast2),
            "ap-southeast-3" => Ok(AwsRegion::ApSouthEast3),
            "ap-southeast-4" => Ok(AwsRegion::ApSouthEast4),
            "ap-northeast-1" => Ok(AwsRegion::ApNorthEast1),
            "ap-northeast-2" => Ok(AwsRegion::ApNorthEast2),
            "ap-northeast-3" => Ok(AwsRegion::ApNorthEast3),
            "cn-north-1" => Ok(AwsRegion::CnNorth1),
            "cn-northwest-1" => Ok(AwsRegion::CnNorthWest1),
            "ca-central-1" => Ok(AwsRegion::CaCentral1),
            "eu-central-1" => Ok(AwsRegion::EuCentral1),
            "eu-central-2" => Ok(AwsRegion::EuCentral2),
            "eu-west-1" => Ok(AwsRegion::EuWest1),
            "eu-west-2" => Ok(AwsRegion::EuWest2),
            "eu-west-3" => Ok(AwsRegion::EuWest3),
            "eu-south-1" => Ok(AwsRegion::EuSouth1),
            "eu-south-2" => Ok(AwsRegion::EuSouth2),
            "eu-north-1" => Ok(AwsRegion::EuNorth1),
            "il-central-1" => Ok(AwsRegion::IlCentral1),
            "me-south-1" => Ok(AwsRegion::MeSouth1),
            "me-central-1" => Ok(AwsRegion::MeCentral1),
            "sa-east-1" => Ok(AwsRegion::SaEast1),
            "us-gov-east-1" => Ok(AwsRegion::UsGovEast1),
            "us-gov-west-1" => Ok(AwsRegion::UsGovWest1),
            _ => Err(String::from("invalid region")),
        }
    }
}

impl AwsRegion {
    pub fn code(&self) -> String {
        match self {
            AwsRegion::USEast1 => String::from("us-east-1"),
            AwsRegion::USEast2 => String::from("us-east-2"),
            AwsRegion::USWest1 => String::from("us-west-1"),
            AwsRegion::USWest2 => String::from("us-west-2"),
            AwsRegion::AfSouth1 => String::from("af-south-1"),
            AwsRegion::ApSouth1 => String::from("ap-south-1"),
            AwsRegion::ApSouth2 => String::from("ap-south-2"),
            AwsRegion::ApEast1 => String::from("ap-east-1"),
            AwsRegion::ApSouthEast1 => String::from("ap-southeast-1"),
            AwsRegion::ApSouthEast2 => String::from("ap-southeast-2"),
            AwsRegion::ApSouthEast3 => String::from("ap-southeast-3"),
            AwsRegion::ApSouthEast4 => String::from("ap-southeast-4"),
            AwsRegion::ApNorthEast1 => String::from("ap-northeast-1"),
            AwsRegion::ApNorthEast2 => String::from("ap-northeast-2"),
            AwsRegion::ApNorthEast3 => String::from("ap-northeast-3"),
            AwsRegion::CnNorth1 => String::from("cn-north-1"),
            AwsRegion::CnNorthWest1 => String::from("cn-northwest-1"),
            AwsRegion::CaCentral1 => String::from("ca-central-1"),
            AwsRegion::EuCentral1 => String::from("eu-central-1"),
            AwsRegion::EuCentral2 => String::from("eu-central-2"),
            AwsRegion::EuWest1 => String::from("eu-west-1"),
            AwsRegion::EuWest2 => String::from("eu-west-2"),
            AwsRegion::EuWest3 => String::from("eu-west-3"),
            AwsRegion::EuSouth1 => String::from("eu-south-1"),
            AwsRegion::EuSouth2 => String::from("eu-south-2"),
            AwsRegion::EuNorth1 => String::from("eu-north-1"),
            AwsRegion::IlCentral1 => String::from("il-central-1"),
            AwsRegion::MeSouth1 => String::from("me-south-1"),
            AwsRegion::MeCentral1 => String::from("me-central-1"),
            AwsRegion::SaEast1 => String::from("sa-east-1"),
            AwsRegion::UsGovEast1 => String::from("us-gov-east-1"),
            AwsRegion::UsGovWest1 => String::from("us-gov-west-1"),
        }
    }

    // region, price for small deployment, price for large deployment
    pub fn to_string_with_price(&self) -> (String, String, (f32, f32)) {
        match self {
            AwsRegion::USEast1 => (
                String::from("US East (N. Virginia)"),
                String::from("us-east-1"),
                (7.14, 427.03),
            ),
            AwsRegion::USEast2 => (
                String::from("US East (Ohio)"),
                String::from("us-east-2"),
                (6.10, 427.03),
            ),
            AwsRegion::USWest1 => (
                String::from("US West (N. California)"),
                String::from("us-west-1"),
                (7.16, 490.96),
            ),
            AwsRegion::USWest2 => (
                String::from("US West (Oregon)"),
                String::from("us-west-2"),
                (7.05, 476.08),
            ),
            AwsRegion::AfSouth1 => (
                String::from("Africa (Cape Town)"),
                String::from("af-south-1"),
                (6.80, 590.08),
            ),
            AwsRegion::ApSouth1 => (
                String::from("Asia Pacific (Mumbai)"),
                String::from("ap-south-1"),
                (4.24, 342.73),
            ),
            AwsRegion::ApSouth2 => (
                String::from("Asia Pacific (Hyderabad)"),
                String::from("ap-south-2"),
                (4.09, 342.73),
            ),
            AwsRegion::ApEast1 => (
                String::from("Asia Pacific (Hong Kong)"),
                String::from("ap-east-1"),
                (8.03, 342.73),
            ),
            AwsRegion::ApSouthEast1 => (
                String::from("Asia Pacific (Singapore)"),
                String::from("ap-southeast-1"),
                (8.27, 544.45),
            ),
            AwsRegion::ApSouthEast2 => (
                String::from("Asia Pacific (Sydney)"),
                String::from("ap-southeast-2"),
                (6.57, 595.54),
            ),
            AwsRegion::ApSouthEast3 => (
                String::from("Asia Pacific (Jakarta)"),
                String::from("ap-southeast-3"),
                (6.57, 556.74),
            ),
            AwsRegion::ApSouthEast4 => (
                String::from("Asia Pacific (Melbourne)"),
                String::from("ap-southeast-4"),
                (6.57, 0.000),
            ),
            AwsRegion::ApNorthEast1 => (
                String::from("Asia Pacific (Tokyo)"),
                String::from("ap-northeast-1"),
                (6.70, 0.0000000000),
            ),
            AwsRegion::ApNorthEast2 => (
                String::from("Asia Pacific (Seoul)"),
                String::from("ap-northeast-2"),
                (6.44, 0.0000000000),
            ),
            AwsRegion::ApNorthEast3 => (
                String::from("Asia Pacific (Osaka)"),
                String::from("ap-northeast-3"),
                (6.66, 0.0000000000),
            ),
            AwsRegion::CnNorth1 => (
                String::from("China (Beijing)"),
                String::from("cn-north-1"),
                (0.0000000000, 0.0000000000),
            ),
            AwsRegion::CnNorthWest1 => (
                String::from("China (Ningxia)"),
                String::from("cn-northwest-1"),
                (0.0000000000, 0.0000000000),
            ),
            AwsRegion::CaCentral1 => (
                String::from("Canada (Central)"),
                String::from("ca-central-1"),
                (5.94, 0.0000000000),
            ),
            AwsRegion::EuCentral1 => (
                String::from("Europe (Frankfurt)"),
                String::from("eu-central-1"),
                (6.20, 0.0000000000),
            ),
            AwsRegion::EuCentral2 => (
                String::from("Europe (Zurich)"),
                String::from("eu-central-2"),
                (6.85, 0.0000000000),
            ),
            AwsRegion::EuWest1 => (
                String::from("Europe (Ireland)"),
                String::from("eu-west-1"),
                (7.61, 0.0000000000),
            ),
            AwsRegion::EuWest2 => (
                String::from("Europe (London)"),
                String::from("eu-west-2"),
                (7.38, 0.0000000000),
            ),
            AwsRegion::EuWest3 => (
                String::from("Europe (Paris)"),
                String::from("eu-west-3"),
                (6.01, 0.0000000000),
            ),
            AwsRegion::EuSouth1 => (
                String::from("Europe (Milan)"),
                String::from("eu-south-1"),
                (6.07, 0.0000000000),
            ),
            AwsRegion::EuSouth2 => (
                String::from("Europe (Spain)"),
                String::from("eu-south-2"),
                (5.77, 0.0000000000),
            ),
            AwsRegion::EuNorth1 => (
                String::from("Europe (Stockholm)"),
                String::from("eu-north-1"),
                (5.42, 0.0000000000),
            ),
            AwsRegion::IlCentral1 => (
                String::from("Israel (Tel Aviv)"),
                String::from("il-central-1"),
                (8.68, 0.0000000000),
            ),
            AwsRegion::MeSouth1 => (
                String::from("Middle East (Bahrain)"),
                String::from("me-south-1"),
                (6.29, 0.0000000000),
            ),
            AwsRegion::MeCentral1 => (
                String::from("Middle East (UAE)"),
                String::from("me-central-1"),
                (6.34, 0.0000000000),
            ),
            AwsRegion::SaEast1 => (
                String::from("South America (Sao Paulo)"),
                String::from("sa-east-1"),
                (8.80, 0.0000000000),
            ),
            AwsRegion::UsGovEast1 => (
                String::from("AWS GovCloud (US-East)"),
                String::from("us-gov-east-1"),
                (8.70, 0.0000000000),
            ),
            AwsRegion::UsGovWest1 => (
                String::from("AWS GovCloud (US-West)"),
                String::from("us-gov-west-1"),
                (8.70, 0.0000000000),
            ),
        }
    }
}
