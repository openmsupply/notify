use std::env;

use serde::Serialize;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TERA_TEMPLATES: Tera = {
        #[allow(unused_mut)]
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        println!(
            "Tera templates loaded: {:?}, {:?}",
            tera.get_template_names().collect::<Vec<_>>(),
            env::current_dir()
        );
        tera
    };
}

#[derive(Debug)]
pub enum RenderError {
    Tera(tera::Error),
    GenericError(String),
}

pub fn render_template(template_name: &str, params: impl Serialize) -> Result<String, RenderError> {
    let context = Context::from_serialize(params)
        .map_err(|e| RenderError::GenericError(format!("Error serializing params: {}", e)))?;

    let rendered = match TERA_TEMPLATES.render(template_name, &context) {
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
        let rendered = render_template("index.html", variables).unwrap();
        assert!(rendered.contains("Hello, world"));
    }
}
