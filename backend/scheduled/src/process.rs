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
    // Check if any scheduled notifications are due

    let scheduled_notifications = ctx
        .service_provider
        .notification_config_service
        .get_notification_configs_by_kind_and_next_check_date(
            ctx,
            NotificationConfigKind::Scheduled,
            current_time,
        )
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;
    let notifications_processed = scheduled_notifications.len();
    for scheduled_notification in scheduled_notifications {
        log::info!(
            "Processing scheduled notification: {} - {}",
            scheduled_notification.id,
            scheduled_notification.title
        );
        match try_process_scheduled_notifications(ctx, scheduled_notification, current_time) {
            // Here, can save against notification config record
            // Can also return next_due_datetime in result and save it here rather then in try_process_scheduled_notification
            // And increment skipped notifications count
            Err(ProcessNotificationError::Skipped(message)) => log::info!("{}", message),
            Err(ProcessNotificationError::NotificationError(error)) => return Err(error),
            Ok(_) => {}
        }
        log::info!("Successfully created notification events");
    }
    // Return the number of notifications processed
    Ok(notifications_processed)
}

enum ProcessNotificationError {
    Skipped(String),
    NotificationError(NotificationError),
}

fn try_process_scheduled_notifications(
    ctx: &ServiceContext,
    scheduled_notification: NotificationConfigRow,
    now: NaiveDateTime,
) -> Result<(), ProcessNotificationError> {
    use ProcessNotificationError as Error;

    // Load the notification config
    let config =
        ScheduledNotificationPluginConfig::from_string(&scheduled_notification.configuration_data)
            .map_err(Error::NotificationError)?;

    let previous_due_datetime = scheduled_notification.next_check_datetime;

    // Get next notification due date
    let next_due_datetime = config
        .next_due_date(DateTime::from_utc(now, Utc))
        .map_err(|e| {
            Error::Skipped(format!(
                "Invalid next due date for scheduled notification: {} - {:?}",
                &scheduled_notification.id, e
            ))
        })?;

    // Update the last_checked time and next_check time
    // We do this before checking if the notification is due so that if the notification is new, we set a good next check time
    NotificationConfigRowRepository::new(&ctx.connection)
        .update_one(&NotificationConfigRow {
            last_check_datetime: Some(now),
            next_check_datetime: Some(next_due_datetime.naive_utc()),
            ..scheduled_notification.clone()
        })
        .map_err(|e| {
            Error::NotificationError(NotificationError::InternalError(format!("{:?}", e)))
        })?;

    // Should notification run ?
    let previous_due_datetime = previous_due_datetime.ok_or(Error::Skipped(format!(
        "No next check time for scheduled notification {}, setting to {}",
        scheduled_notification.id, next_due_datetime
    )))?;

    if previous_due_datetime > now {
        return Err(Error::Skipped(format!(
            "Scheduled notification {} is not due yet, skipping",
            scheduled_notification.id
        )));
    }

    // TODO: Run SQL Queries to get the data https://github.com/openmsupply/notify/issues/137
    // Put sql queries and appropriate data into Json Value for template
    let template_data = serde_json::from_str("{}")
        .map_err(|e| Error::Skipped(format!("Failed to parse template data: {:?}", e)))?;

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

    create_notification_events(ctx, Some(scheduled_notification.id), notification).map_err(
        |e| Error::NotificationError(NotificationError::InternalError(format!("{:?}", e))),
    )?;

    Ok(())
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
