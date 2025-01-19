use minijinja::Environment;

#[derive(Clone)]
pub struct TemplateManager {
    pub templates: Environment<'static>,
}

impl TemplateManager {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_template() {
        let mut manager = TemplateManager::new().unwrap();
        let result = manager.add_template("test", "Hello {{ name }}!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_template() {
        let mut manager = TemplateManager::new().unwrap();
        manager.add_template("test", "Hello {{ name }}!").unwrap();

        let template = manager.get_template("test");
        assert!(template.is_ok());
    }

    #[test]
    fn test_get_nonexistent_template() {
        let manager = TemplateManager::new().unwrap();
        let result = manager.get_template("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_invalid_template() {
        let mut manager = TemplateManager::new().unwrap();
        let result = manager.add_template("invalid", "{{ invalid syntax }");
        assert!(result.is_err());
    }
}
