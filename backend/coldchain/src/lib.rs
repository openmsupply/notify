use chrono::NaiveDateTime;
use serde::Serialize;
use service::{
    notification::{
        self,
        enqueue::{create_notification_events, NotificationContext, NotificationRecipient},
    },
    service_provider::ServiceContext,
};

/*

Temperature Alerts will look something like this...
-----------------------
High temperature alert!

Facility: Store A
Location: Fridge 1
Sensor: E5:4G:D4:6D:A4

Date: 17 Jul 2023
Time: 17:04

Temperature: 10° C
-----------------------
*/

#[derive(Clone, Debug, Serialize)]
pub struct TemperatureAlert {
    pub store_id: String,
    pub store_name: String,
    pub location_id: String,
    pub location_name: String,
    pub sensor_id: String,
    pub sensor_name: String,
    pub datetime: NaiveDateTime,
    pub temperature: f64,
}

pub async fn send_high_temperature_alert_telegram(
    ctx: &ServiceContext,
    alert: TemperatureAlert,
    recipients: Vec<NotificationRecipient>,
) -> Result<(), notification::NotificationServiceError> {
    let notification = NotificationContext {
        title_template_name: None,
        body_template_name: "coldchain/telegram/temperature.html".to_string(),
        recipients,
        template_data: serde_json::to_value(alert).map_err(|e| {
            notification::NotificationServiceError::GenericError(format!(
                "Error serializing template data: {}",
                e
            ))
        })?,
    };

    create_notification_events(ctx, notification)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationEventRowRepository, NotificationType,
    };
    use service::test_utils::get_test_settings;
    use service::test_utils::telegram_test::get_default_telegram_chat_id;
    use service::test_utils::telegram_test::send_test_notifications;

    use crate::notification::enqueue::NotificationRecipient;
    use std::str::FromStr;

    use service::service_provider::ServiceContext;
    use service::service_provider::ServiceProvider;

    use super::*;

    #[tokio::test]
    async fn test_send_high_temperature_alert_telegram() {
        let (_, _, connection_manager, _) =
            setup_all("test_enqueue_telegram", MockDataInserts::none()).await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let example_alert = TemperatureAlert {
            store_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b1".to_string(),
            store_name: "Store A".to_string(),
            location_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b2".to_string(),
            location_name: "Fridge 1".to_string(),
            sensor_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b3".to_string(),
            sensor_name: "E5:4G:D4:6D:A4".to_string(),
            datetime: NaiveDateTime::from_str("2023-07-17T17:04:00").unwrap(),
            temperature: 10.12345,
        };

        let recipient = NotificationRecipient {
            name: "test".to_string(),
            to_address: get_default_telegram_chat_id(),
            notification_type: NotificationType::Telegram,
        };

        let result =
            send_high_temperature_alert_telegram(&context, example_alert.clone(), vec![recipient])
                .await;

        assert!(result.is_ok());

        // Check we have a notification event
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 1);
        assert_eq!(
            notification_event_rows[0].to_address,
            get_default_telegram_chat_id()
        );
        assert!(notification_event_rows[0].title.is_none());
        assert!(notification_event_rows[0]
            .message
            .contains(&example_alert.store_name));

        send_test_notifications(&context).await;
    }
}