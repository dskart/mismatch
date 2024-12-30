use crate::api;
use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Api", default)]
    pub api: api::Config,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        // self.app.validate()?;
        self.api.validate()?;
        Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> Result<()> {
        self.api
            .load_from_env([prefix, "API__"].join("").as_str())?;
        Ok(())
    }
}
