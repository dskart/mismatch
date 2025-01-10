use crate::api::ui::{templating, UiState};
use axum::{extract::State, http::StatusCode, response::Html, routing, Json};
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

pub fn get_router(ui_state: Arc<UiState>) -> anyhow::Result<axum::Router> {
    let router = axum::Router::new()
        .route("/submit", routing::post(handle_post_submit))
        .with_state(ui_state);
    anyhow::Ok(router)
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
    State(state): State<Arc<UiState>>,
    Json(req): Json<PostSubmitRequest>,
) -> Result<Html<String>, StatusCode> {
    // Validate input length
    if req.word1.len() > 20 || req.word2.len() > 20 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate that inputs contain only letters
    if !req.word1.chars().all(char::is_alphabetic) || !req.word2.chars().all(char::is_alphabetic) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let template = state
        .template_state
        .get_template("components/submit.html.j2")
        .unwrap();

    let score = state
        .api_state
        .app
        .get_score(req.word1.clone(), req.word2.clone())
        .unwrap();

    let rendered = template
        .render(context!(
            score => score,
            word1 => req.word1,
            word2 => req.word2
        ))
        .map_err(|e| {
            error!("Failed to render template: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(rendered))
}
