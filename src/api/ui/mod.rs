use std::path::Path;
use tower_http::services::ServeDir;

mod components;
mod pages;
mod templating;

pub fn get_router() -> anyhow::Result<axum::Router> {
    let public_path = Path::new(file!())
        .parent()
        .expect("failed to get current dir")
        .join("public");

    let mut template_state = templating::TemplateState::new()?;
    components::add_templates(&mut template_state)?;

    let router = axum::Router::new().nest_service("/public", ServeDir::new(public_path));
    let router = register_pages(&mut template_state, router)?;

    anyhow::Ok(router)
}

pub trait PageRegistration {
    fn register_template(
        &self,
        template_state: &mut templating::TemplateState,
    ) -> anyhow::Result<()>;

    fn get_router(&self, template_state: templating::TemplateState)
        -> anyhow::Result<axum::Router>;
}

fn register_pages(
    template_state: &mut templating::TemplateState,
    parent_router: axum::Router,
) -> anyhow::Result<axum::Router> {
    let pages: Vec<Box<dyn PageRegistration>> = vec![Box::new(pages::HomePage {})];

    for page in &pages {
        page.register_template(template_state)?;
    }

    let mut router = parent_router;
    for page in pages {
        let page_router = page.get_router(template_state.clone())?;
        router = router.merge(page_router);
    }

    anyhow::Ok(router)
}
