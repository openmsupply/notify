use chrono::NaiveDateTime;
use serde::Serialize;
use service::{
    notification::{
        self,
        enqueue::{
            create_notification_events, NotificationContext, NotificationTarget, TemplateDefinition,
        },
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
pub enum AlertType {
    High,
    Low,
    Ok,
    NoData,
}

#[derive(Clone, Debug, Serialize)]
pub struct ColdchainAlert {
    pub sensor_id: String,
    pub sensor_name: String,
    pub store_name: String,
    pub location_name: String,
    pub datetime: NaiveDateTime,
    pub data_age: String,
    pub temperature: String,
    pub alert_type: AlertType,
}

// Later this function probably won't exist, but serves as a reminder/POC...
pub fn queue_temperature_alert(
    ctx: &ServiceContext,
    config_id: Option<String>,
    alert: ColdchainAlert,
    recipients: Vec<NotificationTarget>,
) -> Result<(), notification::NotificationServiceError> {
    let title_template = match alert.alert_type {
        AlertType::High => Some(TemplateDefinition::TemplateName(
            "coldchain/temperature_title.html".to_string(),
        )),
        AlertType::Low => Some(TemplateDefinition::TemplateName(
            "coldchain/temperature_title.html".to_string(),
        )),
        _ => None,
    };

    let body_template = match alert.alert_type {
        AlertType::High | AlertType::Low => {
            TemplateDefinition::TemplateName("coldchain/temperature.html".to_string())
        }
        AlertType::Ok => TemplateDefinition::TemplateName("coldchain/recovered.html".to_string()),
        AlertType::NoData => TemplateDefinition::TemplateName("coldchain/no_data.html".to_string()),
    };

    let notification = NotificationContext {
        title_template,
        body_template,
        recipients,
        template_data: serde_json::to_value(alert).map_err(|e| {
            notification::NotificationServiceError::InternalError(format!(
                "Error serializing template data: {}",
                e
            ))
        })?,
    };

    create_notification_events(ctx, config_id, notification)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationEventRowRepository, NotificationType,
    };
    use service::test_utils::email_test::send_test_emails;
    use service::test_utils::get_test_settings;
    use service::test_utils::telegram_test::get_default_telegram_chat_id;
    use service::test_utils::telegram_test::send_test_notifications;

    use service::notification::enqueue::NotificationTarget;
    use std::str::FromStr;

    use service::service_provider::ServiceContext;
    use service::service_provider::ServiceProvider;

    use super::*;

    #[tokio::test]
    async fn test_send_high_temperature_alert() {
        let (_, _, connection_manager, _) =
            setup_all("test_send_high_temperature_alert", MockDataInserts::none()).await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let example_alert = ColdchainAlert {
            store_name: "Store A".to_string(),
            location_name: "Fridge 1".to_string(),
            sensor_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b3".to_string(),
            sensor_name: "E5:4G:D4:6D:A4".to_string(),
            datetime: NaiveDateTime::from_str("2023-07-17T17:04:00").unwrap(),
            data_age: "1 minutes".to_string(),
            temperature: 10.12345.to_string(),
            alert_type: AlertType::High,
        };

        let recipient1 = NotificationTarget {
            name: "test".to_string(),
            to_address: get_default_telegram_chat_id(),
            notification_type: NotificationType::Telegram,
        };
        let recipient2 = NotificationTarget {
            name: "test-email".to_string(),
            to_address: "test@example.com".to_string(),
            notification_type: NotificationType::Email,
        };

        let result = queue_temperature_alert(
            &context,
            None,
            example_alert.clone(),
            vec![recipient1, recipient2],
        );

        assert!(result.is_ok());

        // Check we have a notification event
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 2);
        assert_eq!(
            notification_event_rows[0].to_address,
            get_default_telegram_chat_id()
        );
        assert!(notification_event_rows[0]
            .message
            .contains(&example_alert.store_name));

        // Check email recipient
        assert_eq!(notification_event_rows[1].to_address, "test@example.com");
        assert!(notification_event_rows[1]
            .message
            .contains(&example_alert.store_name));

        send_test_notifications(&context).await;
        send_test_emails(&context);
    }

    #[tokio::test]
    async fn test_send_low_temperature_alert() {
        let (_, _, connection_manager, _) =
            setup_all("test_send_low_temperature_alert", MockDataInserts::none()).await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let example_alert = ColdchainAlert {
            store_name: "Store A".to_string(),
            location_name: "Fridge 1".to_string(),
            sensor_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b3".to_string(),
            sensor_name: "E5:4G:D4:6D:A4".to_string(),
            datetime: NaiveDateTime::from_str("2023-07-17T17:04:00").unwrap(),
            data_age: "2 minutes".to_string(),
            temperature: 1.01.to_string(),
            alert_type: AlertType::Low,
        };

        let recipient1 = NotificationTarget {
            name: "test".to_string(),
            to_address: get_default_telegram_chat_id(),
            notification_type: NotificationType::Telegram,
        };
        let recipient2 = NotificationTarget {
            name: "test-email".to_string(),
            to_address: "test@example.com".to_string(),
            notification_type: NotificationType::Email,
        };

        let result = queue_temperature_alert(
            &context,
            None,
            example_alert.clone(),
            vec![recipient1, recipient2],
        );

        assert!(result.is_ok());

        // Check we have a notification event
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 2);
        assert_eq!(
            notification_event_rows[0].to_address,
            get_default_telegram_chat_id()
        );
        assert!(notification_event_rows[0]
            .message
            .contains(&example_alert.store_name));

        // Check email recipient
        assert_eq!(notification_event_rows[1].to_address, "test@example.com");
        assert!(notification_event_rows[1]
            .message
            .contains(&example_alert.store_name));

        send_test_notifications(&context).await;
        send_test_emails(&context);
    }

    #[tokio::test]
    async fn test_no_data_alert() {
        let (_, _, connection_manager, _) =
            setup_all("test_no_data_alert", MockDataInserts::none()).await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        let example_alert = ColdchainAlert {
            store_name: "Store A".to_string(),
            location_name: "Fridge 1".to_string(),
            sensor_id: "6a3399dd-10a9-40b7-853e-3ac0634ce6b3".to_string(),
            sensor_name: "E5:4G:D4:6D:A4".to_string(),
            datetime: NaiveDateTime::from_str("2023-07-17T00:04:00").unwrap(),
            data_age: "60 minutes".to_string(),
            temperature: 1.01.to_string(),
            alert_type: AlertType::NoData,
        };

        let recipient1 = NotificationTarget {
            name: "test".to_string(),
            to_address: get_default_telegram_chat_id(),
            notification_type: NotificationType::Telegram,
        };
        let recipient2 = NotificationTarget {
            name: "test-email".to_string(),
            to_address: "test@example.com".to_string(),
            notification_type: NotificationType::Email,
        };

        let result = queue_temperature_alert(
            &context,
            None,
            example_alert.clone(),
            vec![recipient1, recipient2],
        );

        assert!(result.is_ok());

        // Check we have a notification event
        let notification_event_row_repository = NotificationEventRowRepository::new(&connection);
        let notification_event_rows = notification_event_row_repository.un_sent().unwrap();

        assert_eq!(notification_event_rows.len(), 2);
        assert_eq!(
            notification_event_rows[0].to_address,
            get_default_telegram_chat_id()
        );
        assert!(notification_event_rows[0]
            .message
            .contains(&example_alert.store_name));

        // Check email recipient
        assert_eq!(notification_event_rows[1].to_address, "test@example.com");
        assert!(notification_event_rows[1]
            .message
            .contains(&example_alert.store_name));

        send_test_notifications(&context).await;
        send_test_emails(&context);
    }
}
