use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForceU32(#[serde(deserialize_with = "str_to_u32")] pub u32);

pub fn str_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s == "Inf" {
        Ok(u32::MAX)
    } else {
        s.parse().map_err(D::Error::custom)
    }
}
