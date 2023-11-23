mod utils;

use crate::utils::set_panic_hook;
use core::fmt;
use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    // This is optional to call, but will print panic messages to the console
    set_panic_hook();
}

#[derive(Debug, Clone)]
pub enum TeraRenderError {
    TemplateError(String),
    ParameterError(String),
    RenderError(String),
}
impl std::error::Error for TeraRenderError {}

impl fmt::Display for TeraRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TeraRenderError::TemplateError(s) => write!(f, "Template Error: {}", s),
            TeraRenderError::ParameterError(s) => write!(f, "Parameter Error: {}", s),
            TeraRenderError::RenderError(s) => write!(f, "Render Error: {}", s),
        }
    }
}

impl From<TeraRenderError> for JsValue {
    fn from(e: TeraRenderError) -> Self {
        JsValue::from_str(&format!("{}", e))
    }
}

#[wasm_bindgen(js_name = renderOneOff)]
pub fn render_one_off(template: &str, params: &str) -> Result<String, TeraRenderError> {
    let params: serde_json::Value = serde_json::from_str(params)
        .map_err(|e| TeraRenderError::ParameterError(format!("{}", e)))?;
    let context = Context::from_serialize(params)
        .map_err(|e| TeraRenderError::ParameterError(format!("{}", e)))?;
    let result = Tera::one_off(template, &context, true).map_err(|e| {
        let error_str = format!("{:?}", e);
        if error_str.contains("Failed to parse") {
            return TeraRenderError::TemplateError(error_str);
        }
        if error_str.contains("not found in context while rendering") {
            return TeraRenderError::ParameterError(error_str);
        }

        TeraRenderError::RenderError(error_str)
    })?;
    Ok(result)
}

#[cfg(test)]
mod test {

    use crate::render_one_off;

    #[test]
    fn test_render_one_off() {
        let template = "Hello {{ name }}";
        let params = r#"{"name": "world"}"#;
        let result = render_one_off(template, params).unwrap();
        assert_eq!(result, "Hello world".to_string());
    }

    #[test]
    fn test_render_one_off_template_error() {
        // Broken
        let template = "Hello {{ name }";
        let params = r#"{"name": "world"}"#;
        let result = render_one_off(template, params).unwrap_err();
        print!("{:?}", result);
        assert!(match result {
            crate::TeraRenderError::TemplateError(_) => true,
            _ => {
                println!("{:?}", result);
                false
            }
        });
    }

    #[test]
    fn test_render_one_off_parameter_error() {
        let template = "Hello {{ name }}";
        // Missing parameter
        let params = r#"{"no_name": "world"}"#;
        let result = render_one_off(template, params).unwrap_err();
        assert!(match result {
            crate::TeraRenderError::ParameterError(_) => true,
            _ => {
                println!("{:?}", result);
                false
            }
        });
    }
}
