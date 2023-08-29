use chrono::Utc;
use repository::{
    NotificationEventRow, NotificationEventRowRepository, NotificationEventStatus,
    NotificationType, RecipientRow,
};
use serde::Serialize;
use util::uuid::uuid;

use crate::service_provider::ServiceContext;

use super::NotificationServiceError;

// This struct is intended to be able to be created by a plugin from a datasource, and defines what a template can expect from a recipient
// Often it will be derived RecipientRow which is why we implement From<RecipientRow> for NotificationRecipient
#[derive(Debug, Clone, Serialize)]
pub struct NotificationRecipient {
    pub name: String,
    pub to_address: String,
    pub notification_type: NotificationType,
}

impl From<RecipientRow> for NotificationRecipient {
    fn from(recipient: RecipientRow) -> Self {
        NotificationRecipient {
            name: recipient.name,
            notification_type: recipient.notification_type.into(),
            to_address: recipient.to_address,
        }
    }
}

#[derive(Debug)]
pub struct NotificationContext {
    pub title_template_name: Option<String>,
    pub body_template_name: String,
    pub recipients: Vec<NotificationRecipient>,
    pub template_data: serde_json::Value,
}

pub fn create_notification_events(
    ctx: &ServiceContext,
    notification: NotificationContext,
) -> Result<(), NotificationServiceError> {
    let repo = NotificationEventRowRepository::new(&ctx.connection);

    // Loop through recipients and create a notification for each
    for recipient in notification.recipients {
        let notification_type = recipient.notification_type.clone();

        let mut template_context: serde_json::Value = notification.template_data.clone();
        template_context["recipient"] = serde_json::to_value(recipient.clone()).unwrap_or_default();

        let title = match notification.title_template_name.clone() {
            Some(title_template_name) => Some(
                ctx.service_provider
                    .notification_service
                    .render(&title_template_name, &template_context)
                    .unwrap_or_default(),
            ),
            None => None,
        };

        let message = ctx
            .service_provider
            .notification_service
            .render(&notification.body_template_name.clone(), &template_context);

        let notification_id = uuid();

        let notification_queue_row = match message {
            Ok(message) => NotificationEventRow {
                id: notification_id,
                to_address: recipient.to_address,
                created_at: Utc::now().naive_utc(),
                sent_at: None,
                error_message: None,
                retries: 0,
                updated_at: Utc::now().naive_utc(),
                status: NotificationEventStatus::Queued,
                notification_config_id: None,
                notification_type,
                title,
                message,
                retry_at: None,
            },
            Err(e) => {
                log::error!("Failed to render notification template: {:?}", e);
                NotificationEventRow {
                    id: notification_id,
                    to_address: recipient.to_address,
                    created_at: Utc::now().naive_utc(),
                    sent_at: None,
                    error_message: Some(format!("{:?}", e)),
                    retries: 0,
                    updated_at: Utc::now().naive_utc(),
                    status: NotificationEventStatus::Failed,
                    notification_config_id: None,
                    notification_type,
                    title,
                    message: "".to_string(),
                    retry_at: None,
                }
            }
        };

        let _notification_queue_row = repo
            .insert_one(&notification_queue_row)
            .map_err(|e| NotificationServiceError::DatabaseError(e))?;

        // TODO: trigger async notification send?
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationEventRowRepository, NotificationType,
    };

    use crate::{
        notification::enqueue::{
            create_notification_events, NotificationContext, NotificationRecipient,
        },
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    #[actix_rt::test]
    async fn test_create_notification_events_email() {
        let (_, _, connection_manager, _) = setup_all(
            "test_create_notification_events_email",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let result = create_notification_events(
            &context,
            NotificationContext {
                title_template_name: Some("test_message/email_subject.html".to_string()),
                body_template_name: "test_message/email.html".to_string(),
                recipients: vec![NotificationRecipient {
                    name: "test".to_string(),
                    to_address: "test@example.com".to_string(),
                    notification_type: NotificationType::Email,
                }],
                template_data: serde_json::json!({}),
            },
        );

        assert!(result.is_ok());

        // Check we have a notification event
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 1);
        assert_eq!(
            notification_event_rows[0].to_address,
            "test@example.com".to_string()
        );
        assert!(notification_event_rows[0].title.is_some());
    }

    #[actix_rt::test]
    async fn test_create_notification_events_telegram() {
        let (_, _, connection_manager, _) = setup_all(
            "test_create_notification_events_telegram",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let result = create_notification_events(
            &context,
            NotificationContext {
                title_template_name: None,
                body_template_name: "test_message/telegram.html".to_string(),
                recipients: vec![NotificationRecipient {
                    name: "telegram".to_string(),
                    to_address: "-12345".to_string(),
                    notification_type: NotificationType::Telegram,
                }],
                template_data: serde_json::json!({}),
            },
        );

        assert!(result.is_ok());

        // Check we have a notification event with no title but does have a message
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 1);
        assert_eq!(notification_event_rows[0].to_address, "-12345".to_string());
        assert_ne!(notification_event_rows[0].message, "");
    }
}
