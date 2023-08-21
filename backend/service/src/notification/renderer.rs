use std::env;

use serde::Serialize;
use tera::{Context, Tera};

pub fn tera_with_template(path: Option<String>) -> Tera {
    let path = path.unwrap_or_else(|| "templates/**/*".to_string());

    let tera = match Tera::new(&path) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    log::info!(
        "{} templates loaded from {}/{}",
        tera.get_template_names().count(),
        env::current_dir().unwrap().display(),
        path
    );
    log::debug!(
        "Tera templates loaded: {:?}, {:?}",
        tera.get_template_names().collect::<Vec<_>>(),
        env::current_dir()
    );
    tera
}

#[derive(Debug)]
pub enum RenderError {
    Tera(tera::Error),
    GenericError(String),
}

pub fn render_template(
    tera: &Tera,
    template_name: &str,
    params: impl Serialize,
) -> Result<String, RenderError> {
    let context = Context::from_serialize(params)
        .map_err(|e| RenderError::GenericError(format!("Error serializing params: {}", e)))?;

    let rendered = match tera.render(template_name, &context) {
        Ok(r) => r,
        Err(e) => {
            return Err(RenderError::GenericError(format!(
                "Error rendering template: {}",
                e
            )));
        }
    };

    Ok(rendered)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_template() {
        let variables: serde_json::Value = serde_json::from_str("{\"name\": \"world\"}").unwrap();
        let tera = tera_with_template(None);
        let rendered = render_template(&tera, "index.html", variables).unwrap();
        assert!(rendered.contains("Hello, world"));
    }
}
