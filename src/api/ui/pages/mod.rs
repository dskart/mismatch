use crate::api::ui::templating;
use axum::{extract::State, http::StatusCode, response::Html, routing};
use minijinja::context;
use std::sync::Arc;
use tracing::error;

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

    fn get_router(
        &self,
        template_state: templating::TemplateState,
    ) -> anyhow::Result<axum::Router> {
        let components_router = components::get_router(template_state.clone())?;
        let router = axum::Router::new()
            .route("/", routing::get(handle_index))
            .with_state(Arc::new(template_state))
            .nest("/components", components_router);

        anyhow::Ok(router)
    }
}

async fn handle_index(
    State(state): State<Arc<templating::TemplateState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.templates.get_template("index.html.j2").unwrap();
    let rendered = template.render(context!()).map_err(|e| {
        error!("Failed to render template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}
