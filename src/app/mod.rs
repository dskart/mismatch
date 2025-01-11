use anyhow::Ok;
use model::ModelType;
use ort::session::{builder::GraphOptimizationLevel, Session as OrtSession};
use std::path::Path;
use tokenizers::Tokenizer;

pub mod config;
pub use config::Config;
mod model;
mod score;

#[derive(Debug)]
pub struct App {
    _cfg: Config,
    ort_session: Option<OrtSession>,
    tokenizer: Option<Tokenizer>,
}

impl App {
    pub fn new(cfg: Config) -> anyhow::Result<Self> {
        let (ort_session, tokenizer) = if let ModelType::None = cfg.model {
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
}

fn init_ort(model: String) -> anyhow::Result<(OrtSession, Tokenizer)> {
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

    Ok((ort_session, tokenizer))
}
