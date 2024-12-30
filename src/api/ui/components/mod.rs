use crate::api::ui::templating;

macro_rules! template_key {
    ($name:expr) => {
        concat!("common_components", "/", $name)
    };
}

pub fn add_templates(template_state: &mut templating::TemplateState) -> anyhow::Result<()> {
    template_state.add_template(
        template_key!("header.html.j2"),
        include_str!("header.html.j2"),
    )?;
    Ok(())
}
