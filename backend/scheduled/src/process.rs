use chrono::{NaiveDateTime, Utc};
use repository::{NotificationConfigKind, NotificationConfigRowRepository};
use service::service_provider::ServiceContext;

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

    let now = Utc::now();
    for scheduled_notification in scheduled_notifications {
        notifications_processed += 1;

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
                    scheduled_notification.id,
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
                scheduled_notification.id,
                now.naive_utc(),
                due_datetime.naive_utc(),
            )
            .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

        if due_datetime > now {
            // Not due yet
            continue;
        }
        // Run SQL Queries to get the data

        // Put sql queries data into Json Value for template

        // Send the notification
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
