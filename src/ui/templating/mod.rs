use minijinja::Environment;

#[derive(Clone)]
pub struct TemplateState {
    pub templates: Environment<'static>,
}

impl TemplateState {
    pub fn new() -> anyhow::Result<Self> {
        let env = Environment::new();
        Ok(Self { templates: env })
    }

    pub fn add_template(&mut self, name: &'static str, content: &'static str) -> anyhow::Result<()> {
        self.templates
            .add_template(name, content)
            .map_err(|_| anyhow::anyhow!("failed to add template {}", name))?;
        Ok(())
    }

    pub fn get_template(&self, name: &'static str) -> anyhow::Result<minijinja::Template> {
        self.templates
            .get_template(name)
            .map_err(|e| anyhow::anyhow!("failed to get template {}: {}", name, e))
    }
}
