use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForceF32(#[serde(deserialize_with = "str_to_f32")] pub f32);

pub fn str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    <&str>::deserialize(deserializer).and_then(|s| s.parse().map_err(D::Error::custom))
}
