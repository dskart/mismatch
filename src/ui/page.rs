use crate::ui::pages;
use crate::ui::template_manager::TemplateManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct PageState {
    pub template_manager: Arc<TemplateManager>,
}

pub trait Page {
    fn register_template(&self, template_manager: &mut TemplateManager) -> anyhow::Result<()>;

    fn get_router(&self, ui_state: Arc<PageState>) -> anyhow::Result<axum::Router>;
}

pub fn register_pages(
    template_manager: &mut TemplateManager,
    parent_router: axum::Router,
) -> anyhow::Result<axum::Router> {
    let pages: Vec<Box<dyn Page>> = vec![Box::new(pages::HomePage {})];

    for page in &pages {
        page.register_template(template_manager)?;
    }

    let ui_state = Arc::new(PageState {
        template_manager: Arc::new(template_manager.clone()),
    });

    let mut router = parent_router;
    for page in pages {
        let page_router = page.get_router(ui_state.clone())?;
        router = router.merge(page_router);
    }

    anyhow::Ok(router)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use minijinja::context;

    const TEMPLATE_CONTENT: &str = "Hello {{ name }}!";
    const TEMPLATE_NAME: &str = "test_template.html.j2";

    struct MockPage;

    impl Page for MockPage {
        fn register_template(&self, manager: &mut TemplateManager) -> anyhow::Result<()> {
            manager.add_template(TEMPLATE_NAME, TEMPLATE_CONTENT)
        }

        fn get_router(&self, _: Arc<PageState>) -> anyhow::Result<Router> {
            Ok(Router::new())
        }
    }

    #[tokio::test]
    async fn test_register_pages() -> anyhow::Result<()> {
        let mut template_manager = TemplateManager::new().unwrap();
        let parent_router: Router = Router::new();

        let router = register_pages(&mut template_manager, parent_router);
        assert!(router.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_register_pages_with_mock() -> anyhow::Result<()> {
        let mut template_manager = TemplateManager::new()?;

        let pages: Vec<Box<dyn Page>> = vec![Box::new(MockPage)];
        let ui_state = Arc::new(PageState {
            template_manager: Arc::new(template_manager.clone()),
        });

        for page in &pages {
            assert!(page.register_template(&mut template_manager).is_ok());
            let template = template_manager.get_template(TEMPLATE_NAME)?;
            assert_eq!(template.render(context!(name => "world"))?, "Hello world!");
        }

        for page in pages {
            assert!(page.get_router(ui_state.clone()).is_ok());
        }

        Ok(())
    }
}
