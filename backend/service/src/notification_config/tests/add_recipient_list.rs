#[cfg(test)]
mod notification_config_recipient_list_add_test {
    use repository::mock::{mock_coldchain_notification_config_a, mock_recipient_list_c};
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, NotificationConfigRecipientListFilter,
        NotificationConfigRecipientListRepository, NotificationConfigRecipientListRow,
        NotificationConfigRecipientListRowRepository,
    };
    use std::sync::Arc;

    use crate::notification_config::add_recipient_list::AddRecipientListToNotificationConfig;
    use crate::notification_config::ModifyNotificationConfigError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn add_recipient_list_to_notification_config_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "add_recipient_list_to_notification_config_service_errors",
            MockDataInserts::none()
                .notification_configs()
                .recipient_lists(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // Add recipient_list that doesn't exist
        assert_eq!(
            service.add_recipient_list_to_notification_config(
                &context,
                AddRecipientListToNotificationConfig {
                    recipient_list_id: "some-unknown-id".to_string(),
                    notification_config_id: mock_coldchain_notification_config_a().id.clone(),
                },
            ),
            Err(ModifyNotificationConfigError::RecipientListDoesNotExist)
        );

        // Add recipient_list to notification config that doesn't exist
        assert_eq!(
            service.add_recipient_list_to_notification_config(
                &context,
                AddRecipientListToNotificationConfig {
                    recipient_list_id: mock_recipient_list_c().id.clone(),
                    notification_config_id: "some-unknown-id".to_string(),
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist)
        );

        NotificationConfigRecipientListRowRepository::new(&connection)
            .insert_one(&NotificationConfigRecipientListRow {
                id: "some-random-id".to_string(),
                recipient_list_id: mock_recipient_list_c().id.clone(),
                notification_config_id: mock_coldchain_notification_config_a().id.clone(),
            })
            .unwrap();

        // Add recipient_list to config it is already a part of
        assert_eq!(
            service.add_recipient_list_to_notification_config(
                &context,
                AddRecipientListToNotificationConfig {
                    recipient_list_id: mock_recipient_list_c().id.clone(),
                    notification_config_id: mock_coldchain_notification_config_a().id.clone(),
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigRecipientListAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn add_recipient_list_to_notification_config_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "add_recipient_list_to_notification_config_service_success",
            MockDataInserts::none()
                .recipient_lists()
                .notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let config_recipient_list_repository =
            NotificationConfigRecipientListRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let result = service.add_recipient_list_to_notification_config(
            &context,
            AddRecipientListToNotificationConfig {
                recipient_list_id: mock_recipient_list_c().id.clone(),
                notification_config_id: mock_coldchain_notification_config_a().id.clone(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = config_recipient_list_repository
            .query_one(
                NotificationConfigRecipientListFilter::new()
                    .recipient_list_id(EqualFilter::equal_to(&mock_recipient_list_c().id))
                    .notification_config_id(EqualFilter::equal_to(
                        &mock_coldchain_notification_config_a().id,
                    )),
            )
            .unwrap();

        // NotificationConfigRecipientList now exists
        assert_eq!(
            result.unwrap().recipient_list_id,
            mock_recipient_list_c().id.clone()
        );
    }
}
