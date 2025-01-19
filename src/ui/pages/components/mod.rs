use crate::{
    app,
    ui::page::PageState,
    ui::{error::UiError, template_manager::TemplateManager},
};
use axum::{extract::State, http::StatusCode, response::Html, routing, Extension, Json};
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn get_router(ui_state: Arc<PageState>) -> anyhow::Result<axum::Router> {
    let router = axum::Router::new()
        .route("/submit", routing::post(handle_post_submit))
        .with_state(ui_state);
    anyhow::Ok(router)
}

pub fn add_templates(template_manager: &mut TemplateManager) -> anyhow::Result<()> {
    template_manager.add_template("components/content.html.j2", include_str!("content.html.j2"))?;
    template_manager.add_template("components/high_score.html.j2", include_str!("high_score.html.j2"))?;
    template_manager.add_template(
        "components/score_display.html.j2",
        include_str!("score_display.html.j2"),
    )?;
    template_manager.add_template("components/word_input.html.j2", include_str!("word_input.html.j2"))?;
    anyhow::Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostSubmitRequest {
    pub word1: String,
    pub word2: String,
}

#[axum::debug_handler]
async fn handle_post_submit(
    State(state): State<Arc<PageState>>,
    Extension(session): Extension<Arc<app::Session>>,
    Json(req): Json<PostSubmitRequest>,
) -> Result<Html<String>, UiError> {
    validate_word(&req.word1).map_err(|e| UiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
    validate_word(&req.word2).map_err(|e| UiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let template = state.template_manager.get_template("components/content.html.j2")?;
    let score = session.get_score(req.word1.clone(), req.word2.clone())?;
    let rendered = template.render(context!(
        score => score,
        word1 => req.word1,
        word2 => req.word2
    ))?;

    Ok(Html(rendered))
}

fn validate_word(word: &str) -> anyhow::Result<()> {
    if word.len() > 20 {
        return Err(anyhow::anyhow!("Input too long"));
    }

    if !word.chars().all(char::is_alphabetic) {
        return Err(anyhow::anyhow!("Input must contain only letters"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_word_valid() {
        assert!(validate_word("hello").is_ok());
        assert!(validate_word("Test").is_ok());
        assert!(validate_word("pneumonoultramicroscopicsilicovolcanoconiosis").is_err());
        // Too long
    }

    #[test]
    fn test_validate_word_invalid_chars() {
        assert!(validate_word("hello123").is_err());
        assert!(validate_word("test!").is_err());
        assert!(validate_word("spaces not allowed").is_err());
        assert!(validate_word("").is_ok()); // Empty string is valid
    }

    #[test]
    fn test_validate_word_length() {
        assert!(validate_word("abcdefghijklmnopqrst").is_ok()); // 20 chars
        assert!(validate_word("abcdefghijklmnopqrstu").is_err()); // 21 chars
    }
}
