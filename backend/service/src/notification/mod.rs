use self::renderer::RenderError;
use crate::service_provider::ServiceContext;
use crate::settings::Settings;
use async_trait::async_trait;
use chrono::Utc;
use lettre::address::AddressError;
use repository::{
    NotificationEventRow, NotificationEventRowRepository, NotificationEventStatus,
    NotificationType, RepositoryError,
};
use serde_json::json;
use tera::Tera;

pub mod enqueue;
pub mod renderer;

pub static MAX_RETRIES: i32 = 3;

// We use a trait for NotificationService to allow mocking in tests
#[async_trait(?Send)]
pub trait NotificationServiceTrait: Send + Sync {
    fn render(
        &self,
        template_name: &str,
        params: &serde_json::Value,
    ) -> Result<String, NotificationServiceError>;

    fn render_no_params(&self, template_name: &str) -> Result<String, NotificationServiceError>;

    async fn send_queued_notifications(
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

#[async_trait(?Send)]
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

    async fn send_queued_notifications(
        &self,
        ctx: &ServiceContext,
    ) -> Result<usize, NotificationServiceError> {
        log::debug!("Sending queued notifications");

        let repo = NotificationEventRowRepository::new(&ctx.connection);
        let queued_notifications = repo.un_sent()?;

        let mut error_count = 0;
        let mut sent_count = 0;

        for notification in queued_notifications {
            if notification.notification_type != NotificationType::Telegram {
                log::error!(
                    "Skipping notification type {:?}",
                    notification.notification_type
                );
                //TODO! EMAIL SENDING
                continue;
            }

            let now = Utc::now().naive_utc();

            // Try to send via telegram
            let updated_notification = if let Some(telegram) = &ctx.service_provider.telegram {
                let result = telegram
                    .send_html_message(&notification.to_address, &notification.message)
                    .await;

                match result {
                    Ok(_) => {
                        log::info!("Sent telegram message to {}", notification.to_address);
                        sent_count += 1;
                        NotificationEventRow {
                            error_message: None,
                            status: NotificationEventStatus::Sent,
                            sent_at: Some(now),
                            ..notification
                        }
                    }
                    Err(e) => {
                        sent_count += 1;
                        error_count += 1;
                        log::error!(
                            "Error sending telegram message to {}: {:?}",
                            notification.to_address,
                            e
                        );
                        NotificationEventRow {
                            error_message: Some(format!("{:?}", e)),
                            status: NotificationEventStatus::Failed, //TODO check if this is permanent or temporary failure, retries, and exponential backoff etc
                            ..notification
                        }
                    }
                }
            } else {
                log::error!("Telegram not configured, you are missing telegram notifications!!!!");
                error_count += 1;
                NotificationEventRow {
                    error_message: Some("Telegram Not Configured".to_string()),
                    status: NotificationEventStatus::Errored,
                    ..notification
                }
            };

            repo.update_one(&NotificationEventRow {
                updated_at: now,
                ..updated_notification
            })?;
        }

        log::debug!("Sent {} notifications, {} errors", sent_count, error_count);

        Ok(sent_count)
    }
}
