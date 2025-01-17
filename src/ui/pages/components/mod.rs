use crate::ui::{error::UiError, templating, UiState};
use axum::{extract::State, http::StatusCode, response::Html, routing, Json};
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn get_router(ui_state: Arc<UiState>) -> anyhow::Result<axum::Router> {
    let router = axum::Router::new()
        .route("/submit", routing::post(handle_post_submit))
        .with_state(ui_state);
    anyhow::Ok(router)
}

pub fn add_templates(template_state: &mut templating::TemplateState) -> anyhow::Result<()> {
    template_state.add_template("components/content.html.j2", include_str!("content.html.j2"))?;
    template_state.add_template("components/high_score.html.j2", include_str!("high_score.html.j2"))?;
    template_state.add_template(
        "components/score_display.html.j2",
        include_str!("score_display.html.j2"),
    )?;
    template_state.add_template("components/word_input.html.j2", include_str!("word_input.html.j2"))?;
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
) -> Result<Html<String>, UiError> {
    // Validate input length
    if req.word1.len() > 20 || req.word2.len() > 20 {
        return Err(UiError::new(StatusCode::BAD_REQUEST, "Input too long".to_string()));
    }

    // Validate that inputs contain only letters
    if !req.word1.chars().all(char::is_alphabetic) || !req.word2.chars().all(char::is_alphabetic) {
        return Err(UiError::new(
            StatusCode::BAD_REQUEST,
            "Input must contain only letters".to_string(),
        ));
    }

    let template = state.template_state.get_template("components/content.html.j2")?;

    let score = state.app.get_score(req.word1.clone(), req.word2.clone())?;

    let rendered = template.render(context!(
        score => score,
        word1 => req.word1,
        word2 => req.word2
    ))?;

    Ok(Html(rendered))
}
