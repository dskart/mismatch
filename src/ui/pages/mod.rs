use crate::ui::error::UiError;
use crate::ui::page::PageState;
use crate::ui::template_manager::TemplateManager;
use axum::{extract::State, response::Html, routing};
use minijinja::context;
use std::sync::Arc;
use tracing::info;

use crate::ui::page::Page;

mod components;

pub struct HomePage {}

impl Page for HomePage {
    fn register_template(&self, template_manager: &mut TemplateManager) -> anyhow::Result<()> {
        components::add_templates(template_manager)?;
        template_manager.add_template("index.html.j2", include_str!("index.html.j2"))?;
        anyhow::Ok(())
    }

    fn get_router(&self, ui_state: Arc<PageState>) -> anyhow::Result<axum::Router> {
        let components_router = components::get_router(ui_state.clone())?;
        let router = axum::Router::new()
            .route("/", routing::get(handle_index))
            .with_state(ui_state)
            .nest("/components", components_router);

        anyhow::Ok(router)
    }
}

async fn handle_index(State(state): State<Arc<PageState>>) -> Result<Html<String>, UiError> {
    let template = state.template_manager.get_template("index.html.j2")?;
    let rendered = template.render(context!(
        version => env!("CARGO_PKG_VERSION"),
    ))?;

    info!("Rendered index page");

    Ok(Html(rendered))
}
