use std::{fmt, str::FromStr};

use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Copy, Clone)]
pub enum ModelType {
    #[default]
    None,
    #[serde(rename = "potion-base-8M")]
    PotionBase8M,
}

impl FromStr for ModelType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "potion-base-8M" => Ok(ModelType::PotionBase8M),
            _ => Err(format!("Unknown model type: {}", s)),
        }
    }
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelType::None => write!(f, "none"),
            ModelType::PotionBase8M => write!(f, "potion-base-8M"),
        }
    }
}
