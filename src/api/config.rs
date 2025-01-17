use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "MyVar", default)]
    pub my_var: String,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> Result<()> {
        if let std::result::Result::Ok(my_var) = std::env::var([prefix, "MY_VAR"].join("").as_str()) {
            self.my_var = my_var
                .parse()
                .unwrap_or_else(|_| panic!("could not parse {}", [prefix, "MY_VAR"].join("")));
        }
        Ok(())
    }
}
