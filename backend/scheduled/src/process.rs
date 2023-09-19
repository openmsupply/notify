use chrono::{DateTime, NaiveDateTime, Utc};
use repository::{NotificationConfigKind, NotificationConfigRowRepository, NotificationType};
use service::{
    notification::enqueue::{create_notification_events, NotificationContext, NotificationTarget},
    service_provider::ServiceContext,
};

use crate::{parse::ScheduledNotificationPluginConfig, NotificationError};

pub fn process_scheduled_notifications(
    ctx: &ServiceContext,
    current_time: NaiveDateTime,
) -> Result<usize, NotificationError> {
    log::info!("Processing scheduled notifications due at {}", current_time);
    // Check if any scheduled notifications are due

    let mut notifications_processed = 0;

    let scheduled_notifications = ctx
        .service_provider
        .notification_config_service
        .get_notification_configs_by_kind_and_next_check_date(
            ctx,
            NotificationConfigKind::Scheduled,
            current_time,
        )
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

    let repository = NotificationConfigRowRepository::new(&ctx.connection);

    let now = DateTime::from_utc(current_time, Utc);

    for scheduled_notification in scheduled_notifications {
        notifications_processed += 1;
        log::info!(
            "Processing scheduled notification: {} - {}",
            scheduled_notification.id,
            scheduled_notification.title
        );

        // Load the notification config

        let config = ScheduledNotificationPluginConfig::from_string(
            &scheduled_notification.configuration_data,
        )?;

        // Check if the notification is due
        let due_datetime = match config.next_due_date(now) {
            Ok(dt) => dt,
            Err(e) => {
                log::error!(
                    "Invalid next due date for scheduled notification: {} - {:?}",
                    &scheduled_notification.id,
                    e
                );
                // Log the error but don't fail the whole process
                continue;
            }
        };

        // Update the last_checked time and next_check time
        // We do this before checking if the notification is due so that if the notification is new, we set a good next check time
        repository
            .set_last_checked_and_next_check_date(
                scheduled_notification.id.clone(),
                now.naive_utc(),
                due_datetime.naive_utc(),
            )
            .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

        let next_check_time = match scheduled_notification.next_check_datetime {
            Some(time) => time,
            None => {
                log::info!(
                    "No next check time for scheduled notification {}, setting to {}",
                    scheduled_notification.id,
                    due_datetime
                );
                continue;
            }
        };

        if next_check_time > current_time {
            log::info!(
                "Scheduled notification {} is not due yet, skipping",
                scheduled_notification.id
            );
            continue;
        }

        // For now just send a test notification
        // TODO: Send the actual notification

        // Run SQL Queries to get the data

        // Put sql queries and appropriate data into Json Value for template
        let template_data = match serde_json::from_str("{}") {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to parse template data: {:?}", e);
                continue;
            }
        };

        // Get the recipients
        let recipient1 = NotificationTarget {
            name: "test".to_string(),
            to_address: "test@example.com".to_string(),
            notification_type: NotificationType::Email,
        };

        // Send the notification
        let notification = NotificationContext {
            title_template_name: Some("test_message/email_subject.html".to_string()),
            body_template_name: "test_message/email.html".to_string(),
            template_data: template_data,
            recipients: vec![recipient1],
        };

        match create_notification_events(ctx, Some(scheduled_notification.id), notification) {
            Ok(_) => {
                log::info!("Successfully created notification events");
            }
            Err(e) => {
                log::error!("Error creating notification events: {:?}", e);
                continue;
            }
        };
    }
    // Return the number of notifications processed
    Ok(notifications_processed)
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use service::test_utils::get_test_settings;

    use service::service_provider::ServiceProvider;

    use super::*;

    #[tokio::test]
    async fn test_process_scheduled_notifications() {
        let (_, _, connection_manager, _) =
            setup_all("process_scheduled_notifications", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));

        // Check it runs with no scheduled notifications
        let result = process_scheduled_notifications(
            &ServiceContext::new(service_provider).unwrap(),
            chrono::Utc::now().naive_utc(),
        )
        .unwrap();

        assert_eq!(result, 0);

        // TODO: More tests!
    }
}
