use crate::app;
use std::{path::Path, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod components;
mod error;
mod pages;
mod template_manager;
use template_manager::TemplateManager;
mod page;
use page::register_pages;
mod middleware;

pub fn get_router(app: Arc<app::App>) -> anyhow::Result<axum::Router> {
    let public_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui/public");

    let mut template_manager = TemplateManager::new()?;
    components::add_templates(&mut template_manager)?;

    let router = axum::Router::new().nest_service("/public", ServeDir::new(public_path));
    let router = register_pages(&mut template_manager, router)?;
    let router = router.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(middleware::SessionLayer { app: app.clone() }),
    );
    anyhow::Ok(router)
}
