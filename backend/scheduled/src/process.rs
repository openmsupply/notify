use chrono::{DateTime, NaiveDateTime, Utc};
use repository::{
    NotificationConfigKind, NotificationConfigRow, NotificationConfigRowRepository,
    NotificationType,
};
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

    // Check if any scheduled notifications are due according to the database
    let scheduled_notifications = ctx
        .service_provider
        .notification_config_service
        .find_all_due_by_kind(ctx, NotificationConfigKind::Scheduled, current_time)
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;
    let notifications_processed = scheduled_notifications.len();
    let mut successful_notifications = 0;
    let mut errored_notifications = 0;
    let mut skipped_notifications = 0;
    for scheduled_notification in scheduled_notifications {
        log::info!(
            "Processing scheduled notification: {} - {}",
            scheduled_notification.id,
            scheduled_notification.title
        );
        match try_process_scheduled_notifications(ctx, scheduled_notification, current_time) {
            Err(e) => {
                log::error!("{:?}", e);
                errored_notifications += 1;
            }
            Ok(ProcessingResult::Skipped(message)) => {
                log::info!("{}", message);
                skipped_notifications += 1;
            }
            Ok(ProcessingResult::Success) => {
                log::info!("Successfully created notification events");
                successful_notifications += 1;
            }
        }
    }
    // Return the number of notifications processed
    log::info!(
        "Processed {} out of {} scheduled notifications, skipped {} and errored {}",
        successful_notifications,
        notifications_processed,
        skipped_notifications,
        errored_notifications
    );
    Ok(notifications_processed)
}

enum ProcessingResult {
    Success,
    Skipped(String),
}

fn try_process_scheduled_notifications(
    ctx: &ServiceContext,
    scheduled_notification: NotificationConfigRow,
    now: NaiveDateTime,
) -> Result<ProcessingResult, NotificationError> {
    // Load the notification config
    let config =
        ScheduledNotificationPluginConfig::from_string(&scheduled_notification.configuration_data)?;

    let previous_due_datetime = scheduled_notification.next_check_datetime;

    // Get next notification due date
    let next_due_datetime = config.next_due_date(DateTime::from_utc(now, Utc))?;

    // Update the last_checked time and next_check time
    // We do this before checking if the notification is due so that if the notification is skipped, we still set a good next check time
    NotificationConfigRowRepository::new(&ctx.connection)
        .update_one(&NotificationConfigRow {
            last_check_datetime: Some(now),
            next_check_datetime: Some(next_due_datetime.naive_utc()),
            ..scheduled_notification.clone()
        })
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

    // Should notification run ?
    let previous_due_datetime = match previous_due_datetime {
        Some(dt) => dt,
        None => {
            return Ok(ProcessingResult::Skipped(format!(
                "No next check time for scheduled notification {}, setting to {}",
                scheduled_notification.id, next_due_datetime
            )));
        }
    };

    if previous_due_datetime > now {
        return Ok(ProcessingResult::Skipped(format!(
            "Scheduled notification {} is not due yet, skipping",
            scheduled_notification.id
        )));
    }

    // TODO: Run SQL Queries to get the data https://github.com/openmsupply/notify/issues/137
    // Put sql queries and appropriate data into Json Value for template
    let template_data = serde_json::from_str("{}").map_err(|e| {
        NotificationError::InternalError(format!("Failed to parse template data: {:?}", e))
    })?;

    // TODO: get the real recipients - https://github.com/openmsupply/notify/issues/138
    // Get the recipients
    let recipient1 = NotificationTarget {
        name: "test".to_string(),
        to_address: "test@example.com".to_string(),
        notification_type: NotificationType::Email,
    };

    // For now just send a test notification!
    // TODO: Send the real notification template https://github.com/openmsupply/notify/issues/136
    // Send the notification
    let notification = NotificationContext {
        title_template_name: Some("test_message/email_subject.html".to_string()),
        body_template_name: "test_message/email.html".to_string(),
        template_data: template_data,
        recipients: vec![recipient1],
    };

    create_notification_events(ctx, Some(scheduled_notification.id), notification)
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

    Ok(ProcessingResult::Success)
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

        // Test that it runs with a scheduled notification with no due date
        // Test that it skips a scheduled notification with a due date in the future
        // Test that it runs a scheduled notification with a due date in the past
    }
}
