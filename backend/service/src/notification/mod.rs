use self::renderer::RenderError;
use crate::service_provider::ServiceContext;
use crate::settings::Settings;
use lettre::address::AddressError;
use repository::RepositoryError;
use serde_json::json;
use tera::Tera;

pub mod enqueue;
pub mod renderer;

pub static MAX_RETRIES: i32 = 3;

// We use a trait for NotificationService to allow mocking in tests
pub trait NotificationServiceTrait: Send + Sync {
    fn send_queued_notifications(
        &self,
        ctx: &ServiceContext,
    ) -> Result<(), NotificationServiceError>;

    fn render(
        &self,
        template_name: &str,
        params: &serde_json::Value,
    ) -> Result<String, NotificationServiceError>;

    fn render_no_params(&self, template_name: &str) -> Result<String, NotificationServiceError>;
}

pub struct NotificationService {
    pub tera: Tera,
}

#[derive(Debug)]
pub enum NotificationServiceError {
    GenericError(String),
    RenderError(renderer::RenderError),
    AddressError(AddressError),
    LettreError(lettre::error::Error),
    SmtpError(lettre::transport::smtp::Error),
    DatabaseError(RepositoryError),
}

impl From<RepositoryError> for NotificationServiceError {
    fn from(error: RepositoryError) -> Self {
        NotificationServiceError::DatabaseError(error)
    }
}

impl From<RenderError> for NotificationServiceError {
    fn from(error: RenderError) -> Self {
        NotificationServiceError::RenderError(error)
    }
}

impl NotificationService {
    pub fn new(settings: Settings) -> Self {
        NotificationService {
            tera: renderer::tera_with_template(
                settings
                    .server
                    .base_dir
                    .map(|base_dir| format!("{}/templates/**/*", base_dir)),
            ),
        }
    }
}

impl NotificationServiceTrait for NotificationService {
    fn send_queued_notifications(
        &self,
        ctx: &ServiceContext,
    ) -> Result<(), NotificationServiceError> {
        log::debug!("Sending queued notifications");
        todo!("Implement send_queued_notifications");
        Ok(())
    }

    fn render(
        &self,
        template_name: &str,
        params: &serde_json::Value,
    ) -> Result<String, NotificationServiceError> {
        renderer::render_template(&self.tera, template_name, params).map_err(|e| e.into())
    }

    fn render_no_params(&self, template_name: &str) -> Result<String, NotificationServiceError> {
        renderer::render_template(&self.tera, template_name, json!({})).map_err(|e| e.into())
    }
}
