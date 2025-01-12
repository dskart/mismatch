use crate::api::error::ApiError;
use crate::api::ui::templating;
use crate::api::ui::UiState;
use axum::{extract::State, response::Html, routing};
use minijinja::context;
use std::sync::Arc;

use crate::api::ui::PageRegistration;

mod components;

pub struct HomePage {}

impl PageRegistration for HomePage {
    fn register_template(
        &self,
        template_state: &mut templating::TemplateState,
    ) -> anyhow::Result<()> {
        components::add_templates(template_state)?;
        template_state.add_template("index.html.j2", include_str!("index.html.j2"))?;
        anyhow::Ok(())
    }

    fn get_router(&self, ui_state: Arc<UiState>) -> anyhow::Result<axum::Router> {
        let components_router = components::get_router(ui_state.clone())?;
        let router = axum::Router::new()
            .route("/", routing::get(handle_index))
            .with_state(ui_state)
            .nest("/components", components_router);

        anyhow::Ok(router)
    }
}

async fn handle_index(State(state): State<Arc<UiState>>) -> Result<Html<String>, ApiError> {
    let template = state.template_state.get_template("index.html.j2")?;
    let rendered = template.render(context!())?;

    Ok(Html(rendered))
}
