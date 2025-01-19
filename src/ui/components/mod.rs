use crate::ui::template_manager::TemplateManager;

macro_rules! template_key {
    ($name:expr) => {
        concat!("common_components", "/", $name)
    };
}

pub fn add_templates(template_manager: &mut TemplateManager) -> anyhow::Result<()> {
    template_manager.add_template(template_key!("header.html.j2"), include_str!("header.html.j2"))?;
    Ok(())
}
