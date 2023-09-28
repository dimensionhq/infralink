use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    #[strum(serialize = "aws")]
    Aws,
    #[strum(serialize = "azure")]
    Azure,
    #[strum(serialize = "gcp")]
    Gcp,
    #[strum(disabled)]
    None,
}
