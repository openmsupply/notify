#[cfg(test)]
mod notification_config_update_tests {
    use crate::notification_config::{
        update::UpdateNotificationConfig, ModifyNotificationConfigError,
    };
    use crate::service_provider::{ServiceContext, ServiceProvider};
    use crate::test_utils::get_test_settings;
    use repository::{
        mock::{mock_coldchain_notification_config_a, MockDataInserts},
        test_db::setup_all,
    };
    use std::sync::Arc;

    #[actix_rt::test]
    async fn notification_config_service_update_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_update_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // Trying to updating NotificationConfig that does not exist should fail
        assert_eq!(
            service.update_notification_config(
                &context,
                UpdateNotificationConfig {
                    id: "new_id".to_string(),
                    title: Some("new title".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist)
        );
    }

    #[actix_rt::test]
    async fn notification_config_service_update_success() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_update_success",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // Update title
        let updated_notification_config = context
            .service_provider
            .notification_config_service
            .update_notification_config(
                &context,
                UpdateNotificationConfig {
                    id: mock_coldchain_notification_config_a().id.clone(),
                    title: Some("this is the new title".to_string()),
                    configuration_data: None,
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!(
            updated_notification_config.title,
            "this is the new title".to_string()
        );

        // Update configuration_data
        let updated_notification_config = context
            .service_provider
            .notification_config_service
            .update_notification_config(
                &context,
                UpdateNotificationConfig {
                    id: mock_coldchain_notification_config_a().id.clone(),
                    title: None,
                    configuration_data: Some("{\"confirmOk\":true}".to_string()),
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!(
            updated_notification_config.configuration_data,
            "{\"confirmOk\":true}".to_string()
        );
    }
}
