use std::str::FromStr;

use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Copy, Clone)]
pub enum BlobStorageType {
    #[default]
    #[serde(rename = "S3")]
    S3,
    #[serde(rename = "LOCAL")]
    Local,
}

impl FromStr for BlobStorageType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "LOCAL" => Ok(BlobStorageType::Local),
            "S3" => Ok(BlobStorageType::S3),
            _ => Err(format!("Unknown model type: {}", s)),
        }
    }
}

#[derive(Default, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    #[serde(rename = "BlobStorage")]
    pub blob_storage: BlobStorageType,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> anyhow::Result<()> {
        if let std::result::Result::Ok(blob_storage) = std::env::var([prefix, "BLOB_STORAGE"].join("").as_str()) {
            self.blob_storage = blob_storage
                .parse()
                .unwrap_or_else(|_| panic!("could not parse {}", [prefix, "BLOB_STORAGE"].join("")));
        }
        anyhow::Ok(())
    }
}
