use crate::service_provider::ServiceContext;
use crate::settings::Settings;
use async_trait::async_trait;
use chrono::Utc;
use lettre::address::AddressError;
use repository::{
    NotificationEventRowRepository, NotificationEventStatus, NotificationType, RepositoryError,
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
    RenderError(tera::Error),
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

impl From<tera::Error> for NotificationServiceError {
    fn from(error: tera::Error) -> Self {
        NotificationServiceError::RenderError(error)
    }
}

impl NotificationService {
    pub fn new(settings: Settings) -> Self {
        let template_path = match settings.server.base_dir {
            Some(base_dir) => format!("{}/templates/**/*", base_dir),
            None => "templates/**/*".to_string(), // Assume base dir relative to current dir
        };
        let tera = Tera::new(&template_path)
            .expect(format!("Unable to create tera with path {}", template_path).as_str());

        NotificationService { tera }
    }
}

#[async_trait(?Send)]
impl NotificationServiceTrait for NotificationService {
    fn render(
        &self,
        template_name: &str,
        params: &serde_json::Value,
    ) -> Result<String, NotificationServiceError> {
        Ok(renderer::render_template(
            &self.tera,
            template_name,
            params,
        )?)
    }

    fn render_no_params(&self, template_name: &str) -> Result<String, NotificationServiceError> {
        Ok(renderer::render_template(
            &self.tera,
            template_name,
            json!({}),
        )?)
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

        for mut notification in queued_notifications {
            if notification.notification_type != NotificationType::Telegram {
                log::error!(
                    "Skipping notification type {:?}",
                    notification.notification_type
                );
                //TODO! EMAIL SENDING https://github.com/openmsupply/notify/issues/91
                log::error!("Email notifications not implemented!!!!");
                continue;
            }

            // Try to send via telegram
            if let Some(telegram) = &ctx.service_provider.telegram {
                let result = telegram
                    .send_html_message(&notification.to_address, &notification.message)
                    .await;

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
                        notification.status = NotificationEventStatus::Failed; //TODO check if this is permanent or temporary failure, retries, and exponential backoff etc https://github.com/openmsupply/notify/issues/92
                        notification.updated_at = Utc::now().naive_utc();
                        repo.update_one(&notification)?;
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
