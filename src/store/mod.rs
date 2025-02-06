pub mod config;
pub use config::Config;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod daily_word;

#[derive(Debug)]
pub struct Store {
    _cfg: Config,
    dictionary: Vec<String>,
}

impl Store {
    pub fn new(cfg: Config) -> anyhow::Result<Self> {
        let dict_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("gen")
            .join("english_words.txt");
        let dictionary = load_dictionary(dict_path)?;
        Ok(Store { _cfg: cfg, dictionary })
    }
}

pub fn load_dictionary<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;
    let words: Vec<String> = io::BufReader::new(file).lines().map_while(Result::ok).collect();
    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::config::BlobStorageType;

    pub fn new_test_store() -> Store {
        let cfg = Config {
            blob_storage: BlobStorageType::Local,
        };
        Store::new(cfg).unwrap()
    }
}
