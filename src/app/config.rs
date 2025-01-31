use crate::app::model::ModelType;
use crate::store::Config as StoreConfig;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Copy, Clone)]
pub struct Config {
    #[serde(rename = "Model")]
    pub model: ModelType,

    #[serde(rename = "Store")]
    pub store: StoreConfig,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        self.store.validate()?;
        anyhow::Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> anyhow::Result<()> {
        if let std::result::Result::Ok(model) = std::env::var([prefix, "MODEL"].join("").as_str()) {
            self.model = model
                .parse()
                .unwrap_or_else(|_| panic!("could not parse {}", [prefix, "MODEL"].join("")));
        }

        self.store.load_from_env([prefix, "STORE__"].join("").as_str())?;

        anyhow::Ok(())
    }
}
