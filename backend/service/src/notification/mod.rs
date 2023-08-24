use self::renderer::RenderError;
use crate::service_provider::ServiceContext;
use crate::settings::Settings;
use chrono::Utc;
use lettre::address::AddressError;
use repository::{
    NotificationEventRowRepository, NotificationEventStatus, NotificationType, RepositoryError,
};
use serde_json::json;
use tera::Tera;
use tokio::runtime::Handle;

pub mod enqueue;
pub mod renderer;

pub static MAX_RETRIES: i32 = 3;

// We use a trait for NotificationService to allow mocking in tests
pub trait NotificationServiceTrait: Send + Sync {
    fn render(
        &self,
        template_name: &str,
        params: &serde_json::Value,
    ) -> Result<String, NotificationServiceError>;

    fn render_no_params(&self, template_name: &str) -> Result<String, NotificationServiceError>;

    fn send_queued_notifications(
        &self,
        ctx: &ServiceContext,
    ) -> Result<usize, NotificationServiceError>;
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

    fn send_queued_notifications(
        &self,
        ctx: &ServiceContext,
    ) -> Result<usize, NotificationServiceError> {
        log::debug!("Sending queued notifications");

        // Create a tokio run time so we can run async code.
        // Note we using a current thread runtime, so this will block the current thread until the async code is complete.
        // Which is good in this case, as I think/hope it will prevent multiple copies of this running at once.
        // TODO: Verify this is the case, or add a mutex
        let rt = Handle::try_current();
        let rt = match rt {
            Ok(rt) => rt,
            Err(e) => {
                log::error!("Unable to get tokio runtime from actix: {:?}", e);
                return Err(NotificationServiceError::GenericError(
                    "Unable to get tokio runtime, are you calling this from an actix or tokio runtime?"
                        .to_string(),
                ));
            }
        };

        let repo = NotificationEventRowRepository::new(&ctx.connection);
        let queued_notifications = repo.un_sent()?;
        let mut error_count = 0;
        let mut sent_count = 0;

        for mut notification in queued_notifications {
            if notification.notification_type != NotificationType::Telegram {
                log::error!(
                    "Skipping notification type {:?}",
                    notification.notification_type
                );
                //TODO! EMAIL SENDING
                continue;
            }

            // Try to send via telegram
            if let Some(telegram) = &ctx.service_provider.telegram {
                let result = rt.block_on(
                    telegram.send_html_message(&notification.to_address, &notification.message),
                );
                match result {
                    Ok(_) => {
                        log::info!("Sent telegram message to {}", notification.to_address);
                        notification.error_message = None;
                        notification.status = NotificationEventStatus::Sent;
                        notification.sent_at = Some(Utc::now().naive_utc());
                        notification.updated_at = Utc::now().naive_utc();
                        repo.update_one(&notification)?;
                        sent_count += 1;
                    }
                    Err(e) => {
                        log::error!(
                            "Error sending telegram message to {}: {:?}",
                            notification.to_address,
                            e
                        );
                        notification.error_message = Some(format!("{:?}", e));
                        notification.status = NotificationEventStatus::Failed; //TODO check if this is permanent or temporary failure, retries, and exponential backoff etc
                        notification.updated_at = Utc::now().naive_utc();
                        repo.update_one(&notification)?;
                        sent_count += 1;
                        error_count += 1;
                    }
                }
            } else {
                log::error!("Telegram not configured, you are missing telegram notifications!!!!");
                notification.error_message = Some("Telegram Not Configured".to_string());
                notification.status = NotificationEventStatus::Errored;
                notification.updated_at = Utc::now().naive_utc();
                repo.update_one(&notification)?;
                error_count += 1;
            }
        }

        log::debug!("Sent {} notifications, {} errors", sent_count, error_count);

        Ok(sent_count)
    }
}
