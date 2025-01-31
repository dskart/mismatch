pub mod config;
pub use config::Config;

mod daily_word;

#[derive(Debug)]
pub struct Store {
    _cfg: Config,
}

impl Store {
    pub fn new(cfg: Config) -> anyhow::Result<Self> {
        Ok(Store { _cfg: cfg })
    }
}
