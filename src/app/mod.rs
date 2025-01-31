use anyhow::Ok;
use model::ModelType;
use ort::session::{builder::GraphOptimizationLevel, Session as OrtSession};
use std::{path::Path, sync::Arc};
use tokenizers::Tokenizer;
use tracing::warn;

pub mod config;
pub use config::Config;
mod model;
mod score;
pub mod session;
pub use session::Session;

#[derive(Debug)]
pub struct App {
    _cfg: Config,
    ort_session: Option<Arc<OrtSession>>,
    tokenizer: Option<Arc<Tokenizer>>,
}

impl App {
    pub fn new(cfg: Config) -> anyhow::Result<Self> {
        let (ort_session, tokenizer) = if let ModelType::None = cfg.model {
            warn!("No model specified, skipping model initialization");
            (None, None)
        } else {
            let (ort_session, tokenizers) = init_ort(cfg.model.to_string())?;
            (Some(ort_session), Some(tokenizers))
        };

        Ok(App {
            _cfg: cfg,
            ort_session,
            tokenizer,
        })
    }

    pub fn new_session(&self) -> Session {
        Session::new(self.ort_session.clone(), self.tokenizer.clone())
    }
}

fn init_ort(model: String) -> anyhow::Result<(Arc<OrtSession>, Arc<Tokenizer>)> {
    ort::init().with_name("mismatch").commit()?;
    let ort_session = OrtSession::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .with_intra_threads(1)?
        .commit_from_file(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("models")
                .join(model.clone())
                .join(model.clone() + ".onnx"),
        )?;

    let tokenizer = Tokenizer::from_file(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("models")
            .join(model.clone())
            .join("tokenizer.json"),
    )
    .expect("Failed to load tokenizer");

    let ret = (Arc::new(ort_session), Arc::new(tokenizer));
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub struct TestApp {
        app: App,
    }

    impl TestApp {
        pub fn new(cfg: Option<Config>) -> Self {
            let cfg = cfg.unwrap_or_default();
            let app = App::new(cfg).unwrap();
            Self { app }
        }

        pub fn new_session(&self) -> Session {
            self.app.new_session()
        }
    }
}
