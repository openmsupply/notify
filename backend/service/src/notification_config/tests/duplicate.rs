#[cfg(test)]
mod notification_config_duplicate_tests {
    use crate::notification_config::duplicate::DuplicateNotificationConfig;
    use crate::service_provider::{ServiceContext, ServiceProvider};
    use crate::test_utils::get_test_settings;
    use repository::NotificationConfigStatus;
    use repository::{
        mock::{mock_coldchain_notification_config_a, MockDataInserts},
        test_db::setup_all,
    };
    use std::sync::Arc;

    #[actix_rt::test]
    async fn notification_config_service_duplicate() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_duplicate",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // Duplicated mock_coldchain_notification_config_a() and check..

        // New ID is correctly set in the database

        let _duplicated_notification_config = context
            .service_provider
            .notification_config_service
            .duplicate_notification_config(
                &context,
                DuplicateNotificationConfig {
                    old_id: mock_coldchain_notification_config_a().id.clone(),
                    new_id: "new_id".to_string(),
                },
            )
            .unwrap();

        // Get the duplicated notification config from the database (Check it's saved successfully)
        let duplicated_notification_config = context
            .service_provider
            .notification_config_service
            .get_notification_config(&context, "new_id".to_string())
            .unwrap();

        // Notification is set to disabled
        assert_eq!(
            duplicated_notification_config.status,
            NotificationConfigStatus::Disabled
        );

        // Updated title is set based on the old title
        assert!(duplicated_notification_config
            .title
            .contains(&mock_coldchain_notification_config_a().title));
    }
}
