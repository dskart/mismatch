use std::sync::Arc;

use ort::session::Session as OrtSession;
use tokenizers::Tokenizer;

use crate::store::Store;

pub struct Session {
    pub ort_session: Option<Arc<OrtSession>>,
    pub tokenizer: Option<Arc<Tokenizer>>,
    pub store: Arc<Store>,
}

impl Session {
    pub fn new(ort_session: Option<Arc<OrtSession>>, tokenizer: Option<Arc<Tokenizer>>, store: Arc<Store>) -> Self {
        Self {
            ort_session,
            tokenizer,
            store,
        }
    }
}
