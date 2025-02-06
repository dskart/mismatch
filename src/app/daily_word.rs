use crate::app::session::Session;

impl Session {
    pub fn get_daily_word(&self) -> anyhow::Result<String> {
        self.store.get_daily_word()
    }
}
