use crate::store::Store;
use rand::seq::SliceRandom;

impl Store {
    #[allow(dead_code)]
    pub fn get_daily_word(&self) -> anyhow::Result<String> {
        let mut rng = rand::thread_rng();
        let word = self
            .dictionary
            .choose(&mut rng)
            .ok_or_else(|| anyhow::anyhow!("dictionary is empty"))?;

        Ok(word.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::tests::new_test_store;

    #[test]
    fn test_get_daily_word() {
        let store = new_test_store();
        let word = store.get_daily_word().unwrap();

        assert!(!word.is_empty());
        assert!(store.dictionary.contains(&word));
    }
}
