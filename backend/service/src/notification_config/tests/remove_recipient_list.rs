#[cfg(test)]
mod notification_config_recipient_list_remove_test {
    use repository::mock::{mock_coldchain_notification_config_a, mock_recipient_list_c};
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, NotificationConfigRecipientListFilter,
        NotificationConfigRecipientListRepository, NotificationConfigRecipientListRow,
        NotificationConfigRecipientListRowRepository,
    };
    use std::sync::Arc;

    use crate::notification_config::remove_recipient_list::RemoveRecipientListFromNotifcationConfig;
    use crate::notification_config::ModifyNotificationConfigError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn remove_recipient_list_from_notification_config_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "remove_recipient_list_from_notification_config_service_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // removing recipient_list from config it is not a part of
        assert_eq!(
            service.remove_recipient_list_from_notification_config(
                &context,
                RemoveRecipientListFromNotifcationConfig {
                    recipient_list_id: mock_recipient_list_c().id.clone(),
                    notification_config_id: mock_coldchain_notification_config_a().id.clone(),
                },
            ),
            Err(ModifyNotificationConfigError::NotificationConfigRecipientListDoesNotExist)
        );
    }

    #[actix_rt::test]
    async fn remove_recipient_list_from_notification_config_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "remove_recipient_list_from_notification_config_service_success",
            MockDataInserts::none()
                .recipient_lists()
                .notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let config_recipient_list_repo =
            NotificationConfigRecipientListRepository::new(&connection);
        let config_recipient_list_row_repo =
            NotificationConfigRecipientListRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let recipient_list_id = mock_recipient_list_c().id;
        let notification_config_id = mock_coldchain_notification_config_a().id;

        config_recipient_list_row_repo
            .insert_one(&NotificationConfigRecipientListRow {
                id: "some-random-id".to_string(),
                recipient_list_id: recipient_list_id.clone(),
                notification_config_id: notification_config_id.clone(),
            })
            .unwrap();

        let result = service.remove_recipient_list_from_notification_config(
            &context,
            RemoveRecipientListFromNotifcationConfig {
                recipient_list_id: recipient_list_id.clone(),
                notification_config_id: notification_config_id.clone(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        assert_eq!(
            config_recipient_list_repo
                .query_by_filter(
                    NotificationConfigRecipientListFilter::new()
                        .recipient_list_id(EqualFilter::equal_to(&recipient_list_id))
                        .notification_config_id(EqualFilter::equal_to(&notification_config_id)),
                )
                .unwrap(),
            vec![]
        );
    }
}
