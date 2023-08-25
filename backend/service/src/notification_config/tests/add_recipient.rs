#[cfg(test)]
mod notification_config_recipient_add_test {
    use repository::mock::{mock_coldchain_notification_config_a, mock_recipient_a};
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, NotificationConfigRecipientFilter, NotificationConfigRecipientRepository,
        NotificationConfigRecipientRow, NotificationConfigRecipientRowRepository,
    };
    use std::sync::Arc;

    use crate::notification_config::add_recipient::AddRecipientToNotificationConfig;
    use crate::notification_config::ModifyNotificationConfigError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn add_recipient_to_notification_config_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "add_recipient_to_notification_config_service_errors",
            MockDataInserts::none().notification_configs().recipients(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // Add recipient that doesn't exist
        assert_eq!(
            service.add_recipient_to_notification_config(
                &context,
                AddRecipientToNotificationConfig {
                    recipient_id: "some-unknown-id".to_string(),
                    notification_config_id: mock_coldchain_notification_config_a().id.clone(),
                },
            ),
            Err(ModifyNotificationConfigError::RecipientDoesNotExist)
        );

        // Add recipient to notification config that doesn't exist
        assert_eq!(
            service.add_recipient_to_notification_config(
                &context,
                AddRecipientToNotificationConfig {
                    recipient_id: mock_recipient_a().id.clone(),
                    notification_config_id: "some-unknown-id".to_string(),
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist)
        );

        NotificationConfigRecipientRowRepository::new(&connection)
            .insert_one(&NotificationConfigRecipientRow {
                id: "some-random-id".to_string(),
                recipient_id: mock_recipient_a().id.clone(),
                notification_config_id: mock_coldchain_notification_config_a().id.clone(),
            })
            .unwrap();

        // Add recipient to config it is already a part of
        assert_eq!(
            service.add_recipient_to_notification_config(
                &context,
                AddRecipientToNotificationConfig {
                    recipient_id: mock_recipient_a().id.clone(),
                    notification_config_id: mock_coldchain_notification_config_a().id.clone(),
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigRecipientAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn add_recipient_to_notification_config_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "add_recipient_to_notification_config_service_success",
            MockDataInserts::none().recipients().notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let config_recipient_repository = NotificationConfigRecipientRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let result = service.add_recipient_to_notification_config(
            &context,
            AddRecipientToNotificationConfig {
                recipient_id: mock_recipient_a().id.clone(),
                notification_config_id: mock_coldchain_notification_config_a().id.clone(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = config_recipient_repository
            .query_one(
                NotificationConfigRecipientFilter::new()
                    .recipient_id(EqualFilter::equal_to(&mock_recipient_a().id))
                    .notification_config_id(EqualFilter::equal_to(
                        &mock_coldchain_notification_config_a().id,
                    )),
            )
            .unwrap();

        // NotificationConfigRecipient now exists
        assert_eq!(result.unwrap().recipient_id, mock_recipient_a().id.clone());
    }
}
