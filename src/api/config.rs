use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Foo", default)]
    pub foo: String,
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn load_from_env(&mut self, prefix: &str) -> Result<()> {
        if let std::result::Result::Ok(foo) = std::env::var([prefix, "FOO"].join("").as_str()) {
            self.foo = foo
                .parse()
                .expect(format!("could not parse {}", [prefix, "FOO"].join("")).as_str());
        }
        Ok(())
    }
}
