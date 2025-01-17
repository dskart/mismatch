use crate::app;
use std::{path::Path, sync::Arc};
use tower_http::services::ServeDir;

mod components;
mod error;
mod pages;
mod templating;

pub fn get_router(app: Arc<app::App>) -> anyhow::Result<axum::Router> {
    let public_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui/public");

    let mut template_state = templating::TemplateState::new()?;
    components::add_templates(&mut template_state)?;

    let router = axum::Router::new().nest_service("/public", ServeDir::new(public_path));
    let router = register_pages(app, &mut template_state, router)?;

    anyhow::Ok(router)
}

pub trait PageRegistration {
    fn register_template(&self, template_state: &mut templating::TemplateState) -> anyhow::Result<()>;

    fn get_router(&self, ui_state: Arc<UiState>) -> anyhow::Result<axum::Router>;
}

pub struct UiState {
    pub app: Arc<app::App>,
    pub template_state: Arc<templating::TemplateState>,
}

fn register_pages(
    app: Arc<app::App>,
    template_state: &mut templating::TemplateState,
    parent_router: axum::Router,
) -> anyhow::Result<axum::Router> {
    let pages: Vec<Box<dyn PageRegistration>> = vec![Box::new(pages::HomePage {})];

    for page in &pages {
        page.register_template(template_state)?;
    }

    let ui_state = Arc::new(UiState {
        app,
        template_state: Arc::new(template_state.clone()),
    });
    let mut router = parent_router;
    for page in pages {
        let page_router = page.get_router(ui_state.clone())?;
        router = router.merge(page_router);
    }

    anyhow::Ok(router)
}
