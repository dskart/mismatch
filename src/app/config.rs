use crate::app::model::ModelType;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Model")]
    pub model: ModelType,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> anyhow::Result<()> {
        if let std::result::Result::Ok(model) = std::env::var([prefix, "MODEL"].join("").as_str()) {
            self.model = model
                .parse()
                .unwrap_or_else(|_| panic!("could not parse {}", [prefix, "MODEL"].join("")));
        }
        anyhow::Ok(())
    }
}
