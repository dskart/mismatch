use crate::api;
use crate::app;
use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Api", default)]
    pub api: api::Config,
    #[serde(rename = "App", default)]
    pub app: app::Config,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        self.api.validate()?;
        self.app.validate()?;
        Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> Result<()> {
        self.api.load_from_env([prefix, "API__"].join("").as_str())?;
        self.app.load_from_env([prefix, "APP__"].join("").as_str())?;
        Ok(())
    }
}
