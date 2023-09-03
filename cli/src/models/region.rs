use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Region {
    Aws(AwsRegion),
    None,
}

impl Region {
    pub fn code(&self) -> Option<String> {
        match self {
            Region::Aws(aws_region) => Some(aws_region.code()),
            Region::None => None,
        }
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
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
            _ => Err(format!("{} is an invalid region", s)),
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
        }
    }

    // region, price for small deployment, price for large deployment
    pub fn display_name(&self) -> String {
        match self {
            AwsRegion::USEast1 => String::from("US East (N. Virginia)"),
            AwsRegion::USEast2 => String::from("US East (Ohio)"),
            AwsRegion::USWest1 => String::from("US West (N. California)"),
            AwsRegion::USWest2 => String::from("US West (Oregon)"),
            AwsRegion::AfSouth1 => String::from("Africa (Cape Town)"),
            AwsRegion::ApSouth1 => String::from("Asia Pacific (Mumbai)"),
            AwsRegion::ApSouth2 => String::from("Asia Pacific (Hyderabad)"),
            AwsRegion::ApEast1 => String::from("Asia Pacific (Hong Kong)"),
            AwsRegion::ApSouthEast1 => String::from("Asia Pacific (Singapore)"),
            AwsRegion::ApSouthEast2 => String::from("Asia Pacific (Sydney)"),
            AwsRegion::ApSouthEast3 => String::from("Asia Pacific (Jakarta)"),
            AwsRegion::ApSouthEast4 => String::from("Asia Pacific (Melbourne)"),
            AwsRegion::ApNorthEast1 => String::from("Asia Pacific (Tokyo)"),
            AwsRegion::ApNorthEast2 => String::from("Asia Pacific (Seoul)"),
            AwsRegion::ApNorthEast3 => String::from("Asia Pacific (Osaka)"),
            AwsRegion::CaCentral1 => String::from("Canada (Central)"),
            AwsRegion::EuCentral1 => String::from("Europe (Frankfurt)"),
            AwsRegion::EuCentral2 => String::from("Europe (Zurich)"),
            AwsRegion::EuWest1 => String::from("Europe (Ireland)"),
            AwsRegion::EuWest2 => String::from("Europe (London)"),
            AwsRegion::EuWest3 => String::from("Europe (Paris)"),
            AwsRegion::EuSouth1 => String::from("Europe (Milan)"),
            AwsRegion::EuSouth2 => String::from("Europe (Spain)"),
            AwsRegion::EuNorth1 => String::from("Europe (Stockholm)"),
            AwsRegion::IlCentral1 => String::from("Israel (Tel Aviv)"),
            AwsRegion::MeSouth1 => String::from("Middle East (Bahrain)"),
            AwsRegion::MeCentral1 => String::from("Middle East (UAE)"),
            AwsRegion::SaEast1 => String::from("South America (Sao Paulo)"),
        }
    }

    pub fn from_display_name(name: &str) -> Result<Self, String> {
        match name {
            "US East (N. Virginia)" => Ok(AwsRegion::USEast1),
            "US East (Ohio)" => Ok(AwsRegion::USEast2),
            "US West (N. California)" => Ok(AwsRegion::USWest1),
            "US West (Oregon)" => Ok(AwsRegion::USWest2),
            "Africa (Cape Town)" => Ok(AwsRegion::AfSouth1),
            "Asia Pacific (Mumbai)" => Ok(AwsRegion::ApSouth1),
            "Asia Pacific (Hyderabad)" => Ok(AwsRegion::ApSouth2),
            "Asia Pacific (Hong Kong)" => Ok(AwsRegion::ApEast1),
            "Asia Pacific (Singapore)" => Ok(AwsRegion::ApSouthEast1),
            "Asia Pacific (Sydney)" => Ok(AwsRegion::ApSouthEast2),
            "Asia Pacific (Jakarta)" => Ok(AwsRegion::ApSouthEast3),
            "Asia Pacific (Melbourne)" => Ok(AwsRegion::ApSouthEast4),
            "Asia Pacific (Tokyo)" => Ok(AwsRegion::ApNorthEast1),
            "Asia Pacific (Seoul)" => Ok(AwsRegion::ApNorthEast2),
            "Asia Pacific (Osaka)" => Ok(AwsRegion::ApNorthEast3),
            "Canada (Central)" => Ok(AwsRegion::CaCentral1),
            "Europe (Frankfurt)" => Ok(AwsRegion::EuCentral1),
            "Europe (Zurich)" => Ok(AwsRegion::EuCentral2),
            "Europe (Ireland)" => Ok(AwsRegion::EuWest1),
            "Europe (London)" => Ok(AwsRegion::EuWest2),
            "Europe (Paris)" => Ok(AwsRegion::EuWest3),
            "Europe (Milan)" => Ok(AwsRegion::EuSouth1),
            "Europe (Spain)" => Ok(AwsRegion::EuSouth2),
            "Europe (Stockholm)" => Ok(AwsRegion::EuNorth1),
            "Israel (Tel Aviv)" => Ok(AwsRegion::IlCentral1),
            "Middle East (Bahrain)" => Ok(AwsRegion::MeSouth1),
            "Middle East (UAE)" => Ok(AwsRegion::MeCentral1),
            "South America (Sao Paulo)" => Ok(AwsRegion::SaEast1),
            _ => Err(String::from("Invalid display name")),
        }
    }
}

// Custom serialization
impl Serialize for AwsRegion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.code())
    }
}

// Custom deserialization
impl<'de> Deserialize<'de> for AwsRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AwsRegion::from_str(&s).map_err(serde::de::Error::custom)
    }
}
