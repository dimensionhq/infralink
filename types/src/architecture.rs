use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Architecture {
    X86,
    Arm64,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::X86 => write!(f, "x86"),
            Architecture::Arm64 => write!(f, "arm64"),
        }
    }
}
