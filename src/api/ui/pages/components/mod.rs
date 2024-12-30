use crate::api::ui::templating;
use axum::{extract::State, http::StatusCode, response::Html, routing, Json};
use minijinja::context;
use ndarray::{Axis, Ix2};
use ort::{
    execution_providers::CUDAExecutionProvider,
    session::{builder::GraphOptimizationLevel, Session},
    value::TensorRef,
    Error,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokenizers::Tokenizer;
use tracing::{error, info};

pub fn get_router(template_state: templating::TemplateState) -> anyhow::Result<axum::Router> {
    let router = axum::Router::new()
        .route("/submit", routing::post(handle_post_submit))
        .with_state(Arc::new(template_state));
    return anyhow::Ok(router);
}

pub fn add_templates(template_state: &mut templating::TemplateState) -> anyhow::Result<()> {
    template_state.add_template("components/submit.html.j2", include_str!("submit.html.j2"))?;
    anyhow::Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostSubmitRequest {
    pub word1: String,
    pub word2: String,
}

async fn handle_post_submit(
    State(state): State<Arc<templating::TemplateState>>,
    Json(req): Json<PostSubmitRequest>,
) -> Result<Html<String>, StatusCode> {
    info!("Received request: {:?}", req);

    let template = state
        .templates
        .get_template("components/submit.html.j2")
        .unwrap();
    let rendered = template
        .render(context!(
            score => 100.0,
            word1 => req.word1,
            word2 => req.word2
        ))
        .or_else(|e| {
            error!("Failed to render template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    Ok(Html(rendered))
}

fn get_score(word1: &str, word2: &str) -> anyhow::Result<f64> {
    ort::init().with_name("sbert").commit()?;

    return Ok(100.0);
}
