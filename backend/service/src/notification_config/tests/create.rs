#[cfg(test)]
mod notification_config_create_test {
    use repository::mock::mock_notification_config_a;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{NotificationConfigKind, NotificationConfigRowRepository};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::notification_config::create::CreateNotificationConfig;
    use crate::notification_config::ModifyNotificationConfigError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn create_notification_config_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "create_notification_config_service_errors",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        //Create for a id that already exists
        assert_eq!(
            service.create_notification_config(
                &context,
                CreateNotificationConfig {
                    id: mock_notification_config_a().id.clone(),
                    title: "some title".to_string(),
                    kind: NotificationConfigKind::ColdChain,
                    configuration_data: "{ \"data\": \"some data\" }".to_string()
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn create_notification_config_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "create_notification_config_service_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let notification_config_row_repository = NotificationConfigRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let new_notification_config_id = uuid();
        let result = service.create_notification_config(
            &context,
            CreateNotificationConfig {
                id: new_notification_config_id.clone(),
                title: "new_notification_config".to_string(),
                kind: NotificationConfigKind::ColdChain,
                configuration_data: "{ \"data\": \"some data\" }".to_string(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = notification_config_row_repository
            .find_one_by_id(&new_notification_config_id)
            .unwrap();

        // NotificationConfig now exists
        assert!(result.is_some());
    }
}
