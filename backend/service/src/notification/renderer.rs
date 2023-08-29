use serde::Serialize;
use tera::{Context, Tera};

pub fn render_template(
    tera: &Tera,
    template_name: &str,
    params: impl Serialize,
) -> Result<String, tera::Error> {
    let context = Context::from_serialize(params)?;

    let rendered = tera.render(template_name, &context)?;

    Ok(rendered)
}

#[cfg(test)]
mod test {
    use crate::test_utils::find_base_dir;

    use super::*;

    #[test]
    fn test_render_template() {
        let variables: serde_json::Value = serde_json::from_str("{\"name\": \"world\"}").unwrap();
        let test_templates_path = find_base_dir().to_str().unwrap().to_string();
        let tera = Tera::new(format!("{}/templates/**/*", test_templates_path).as_str()).unwrap();
        let rendered = render_template(&tera, "hello_world.html", variables).unwrap();
        assert!(rendered.contains("Hello, world"));
    }
}
