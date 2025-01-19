use std::sync::Arc;

use ort::session::Session as OrtSession;
use tokenizers::Tokenizer;

pub struct Session {
    pub ort_session: Option<Arc<OrtSession>>,
    pub tokenizer: Option<Arc<Tokenizer>>,
}

impl Session {
    pub fn new(ort_session: Option<Arc<OrtSession>>, tokenizer: Option<Arc<Tokenizer>>) -> Self {
        Self { ort_session, tokenizer }
    }
}
