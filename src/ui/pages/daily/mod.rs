use crate::ui::error::UiError;
use crate::ui::page::PageState;
use crate::ui::template_manager::TemplateManager;
use axum::{extract::State, response::Html, routing};
use minijinja::context;
use std::sync::Arc;

use crate::ui::page::Page;

const PAGE_PREFIX: &str = "/daily";
const TEMPLATE_NAME: &str = "/daily/index.html.j2";

pub struct DailyPage {}

impl Page for DailyPage {
    fn register_template(&self, template_manager: &mut TemplateManager) -> anyhow::Result<()> {
        template_manager.add_template(TEMPLATE_NAME, include_str!("index.html.j2"))?;
        anyhow::Ok(())
    }

    fn get_router(&self, ui_state: Arc<PageState>) -> anyhow::Result<axum::Router> {
        let router = axum::Router::new()
            .route(PAGE_PREFIX, routing::get(handle_index))
            .with_state(ui_state);

        anyhow::Ok(router)
    }
}

async fn handle_index(State(state): State<Arc<PageState>>) -> Result<Html<String>, UiError> {
    let template = state.template_manager.get_template(TEMPLATE_NAME)?;
    let rendered = template.render(context!(
        version => env!("CARGO_PKG_VERSION"),
    ))?;

    Ok(Html(rendered))
}
