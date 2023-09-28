use chrono::{DateTime, NaiveDateTime, Utc};
use repository::{NotificationConfigKind, NotificationConfigRowRepository};
use service::{
    notification::enqueue::{create_notification_events, NotificationContext, TemplateDefinition},
    notification_config::{query::NotificationConfig, recipients::get_notification_targets},
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

#[derive(Debug, PartialEq)]
enum ProcessingResult {
    Success,
    Skipped(String),
}

fn try_process_scheduled_notifications(
    ctx: &ServiceContext,
    scheduled_notification: NotificationConfig,
    now: NaiveDateTime,
) -> Result<ProcessingResult, NotificationError> {
    // Load the notification config
    let config =
        ScheduledNotificationPluginConfig::from_string(&scheduled_notification.configuration_data)?;

    let previous_due_datetime = scheduled_notification.next_due_datetime;

    // Get next notification due date
    let next_due_datetime = config.next_due_date(DateTime::from_utc(now, Utc))?;

    // Update the last_checked time and next_check time
    // We do this before checking if the notification is due so that if the notification is skipped, we still set a good next check time
    NotificationConfigRowRepository::new(&ctx.connection)
        .set_next_due_by_id(
            &scheduled_notification.id,
            Some(now),
            Some(next_due_datetime.naive_utc()),
        )
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

    // Get the recipients
    let notification_targets =
        get_notification_targets(ctx, &scheduled_notification).map_err(|e| {
            NotificationError::InternalError(format!("Failed to get notification targets: {:?}", e))
        })?;

    // Send the notification
    let notification = NotificationContext {
        title_template: Some(TemplateDefinition::Template(config.subject_template)),
        body_template: TemplateDefinition::Template(config.body_template),
        template_data: template_data,
        recipients: notification_targets,
    };

    create_notification_events(ctx, Some(scheduled_notification.id), notification)
        .map_err(|e| NotificationError::InternalError(format!("{:?}", e)))?;

    Ok(ProcessingResult::Success)
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use chrono::Days;
    use repository::mock::{
        mock_recipient_a, mock_recipient_list_with_recipient_members_a_and_b,
        mock_sql_recipient_list_with_no_param, mock_sql_recipient_list_with_param,
    };
    use repository::NotificationEventRowRepository;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use service::test_utils::get_test_settings;

    use service::service_provider::ServiceProvider;
    use service::test_utils::telegram_test::send_test_notifications;

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

    // Test that we get a notification when we have a recipient list configured
    #[tokio::test]
    async fn test_try_process_scheduled_notifications_with_recipient_list() {
        let (_, _, connection_manager, _) = setup_all(
            "test_try_process_scheduled_notifications_with_recipient_list",
            MockDataInserts::none()
                .recipients()
                .recipient_lists()
                .recipient_list_members(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));

        let service_context = ServiceContext::new(service_provider).unwrap();

        // Daily Scheduled Notification that started this time yesterday
        let sch_config = ScheduledNotificationPluginConfig {
            body_template: "TestTemplate".to_string(),
            subject_template: "TestSubject".to_string(),
            schedule_frequency: "daily".to_string(),
            schedule_start_time: Utc::now().checked_sub_days(Days::new(1)).unwrap(),
        };

        // Create a notification config with a recipient list
        let notification_config = NotificationConfig {
            id: "notification_config_1".to_string(),
            kind: NotificationConfigKind::Scheduled,
            recipient_ids: vec![],
            recipient_list_ids: vec![mock_recipient_list_with_recipient_members_a_and_b().id],
            sql_recipient_list_ids: vec![],
            next_due_datetime: Some(chrono::Utc::now().naive_utc()),
            configuration_data: serde_json::to_string(&sch_config).unwrap(),
            ..Default::default()
        };

        // Try to process the notification (Should be due now)
        let result = try_process_scheduled_notifications(
            &service_context,
            notification_config,
            chrono::Utc::now().naive_utc(),
        )
        .unwrap();

        assert_eq!(result, ProcessingResult::Success);

        // Query the database to check that we have a notification events for each recipient
        let repo = NotificationEventRowRepository::new(&service_context.connection);
        let notification_events = repo.un_sent().unwrap();

        assert_eq!(notification_events.len(), 2);

        send_test_notifications(&service_context).await;
    }

    // Test that we get a notification when we have a recipient configured
    #[tokio::test]
    async fn test_try_process_scheduled_notifications_with_recipient() {
        let (_, _, connection_manager, _) = setup_all(
            "test_try_process_scheduled_notifications_with_recipient",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));

        let service_context = ServiceContext::new(service_provider).unwrap();

        // Daily Scheduled Notification that started this time yesterday
        let sch_config = ScheduledNotificationPluginConfig {
            body_template: "TestTemplate".to_string(),
            subject_template: "TestSubject".to_string(),
            schedule_frequency: "daily".to_string(),
            schedule_start_time: Utc::now().checked_sub_days(Days::new(1)).unwrap(),
        };

        // Create a notification config with a recipient list
        let notification_config = NotificationConfig {
            id: "notification_config_1".to_string(),
            kind: NotificationConfigKind::Scheduled,
            recipient_ids: vec![mock_recipient_a().id],
            recipient_list_ids: vec![],
            sql_recipient_list_ids: vec![],
            next_due_datetime: Some(chrono::Utc::now().naive_utc()),
            configuration_data: serde_json::to_string(&sch_config).unwrap(),
            ..Default::default()
        };

        // Try to process the notification (Should be due now)
        let result = try_process_scheduled_notifications(
            &service_context,
            notification_config,
            chrono::Utc::now().naive_utc(),
        )
        .unwrap();

        assert_eq!(result, ProcessingResult::Success);

        // Query the database to check that we have a notification events for the 1 configured recipient
        let repo = NotificationEventRowRepository::new(&service_context.connection);
        let notification_events = repo.un_sent().unwrap();

        assert_eq!(notification_events.len(), 1);
    }

    // Test that we get a notification when we have a sql recipient list configured
    #[tokio::test]
    async fn test_try_process_scheduled_notifications_with_sql_recipients() {
        let (_, _, connection_manager, _) = setup_all(
            "test_try_process_scheduled_notifications_with_sql_recipients",
            MockDataInserts::none().sql_recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));

        let service_context = ServiceContext::new(service_provider).unwrap();

        // Daily Scheduled Notification that started this time yesterday
        let sch_config = ScheduledNotificationPluginConfig {
            body_template: "TestTemplate".to_string(),
            subject_template: "TestSubject".to_string(),
            schedule_frequency: "daily".to_string(),
            schedule_start_time: Utc::now().checked_sub_days(Days::new(1)).unwrap(),
        };

        // Create a notification config with 2 sql recipient lists
        let notification_config = NotificationConfig {
            id: "notification_config_1".to_string(),
            kind: NotificationConfigKind::Scheduled,
            recipient_ids: vec![],
            recipient_list_ids: vec![],
            sql_recipient_list_ids: vec![
                mock_sql_recipient_list_with_no_param().id,
                mock_sql_recipient_list_with_param().id,
            ],
            parameters: "{ \"email_address\": \"test-user@example.com\"}".to_string(),
            next_due_datetime: Some(chrono::Utc::now().naive_utc()),
            configuration_data: serde_json::to_string(&sch_config).unwrap(),
            ..Default::default()
        };

        // Try to process the notification (Should be due now)
        let result = try_process_scheduled_notifications(
            &service_context,
            notification_config,
            chrono::Utc::now().naive_utc(),
        )
        .unwrap();

        assert_eq!(result, ProcessingResult::Success);

        // Query the database to check that we have a notification events for the 1 configured recipient
        let repo = NotificationEventRowRepository::new(&service_context.connection);
        let notification_events = repo.un_sent().unwrap();

        assert_eq!(notification_events.len(), 2);
    }
}
