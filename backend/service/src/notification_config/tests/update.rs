#[cfg(test)]
mod notification_config_update_tests {
    use crate::notification_config::{
        update::UpdateNotificationConfig, ModifyNotificationConfigError,
    };
    use crate::service_provider::{ServiceContext, ServiceProvider};
    use crate::test_utils::get_test_settings;
    use repository::mock::{
        mock_recipient_a, mock_recipient_aa, mock_recipient_b,
        mock_recipient_list_b_with_no_members, mock_recipient_list_c, mock_recipient_list_c2,
    };
    use repository::{
        mock::{mock_coldchain_notification_config_a, MockDataInserts},
        test_db::setup_all,
    };
    use repository::{
        EqualFilter, NotificationConfigRecipientFilter, NotificationConfigRecipientListFilter,
        NotificationConfigRecipientListRepository, NotificationConfigRecipientListRow,
        NotificationConfigRecipientListRowRepository, NotificationConfigRecipientRepository,
        NotificationConfigRecipientRow, NotificationConfigRecipientRowRepository,
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
                    configuration_data: None,
                    recipient_ids: None,
                    recipient_list_ids: None,
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
                    recipient_ids: None,
                    recipient_list_ids: None,
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
                    recipient_ids: None,
                    recipient_list_ids: None,
                },
            )
            .unwrap();

        assert_eq!(
            updated_notification_config.configuration_data,
            "{\"confirmOk\":true}".to_string()
        );
    }

    #[actix_rt::test]
    async fn notification_config_service_update_recipient_ids() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_update_recipient_ids",
            MockDataInserts::none().recipients().notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let config_recipient_repo = NotificationConfigRecipientRepository::new(&connection);
        let config_recipient_row_repo = NotificationConfigRecipientRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // Add recipient A and B to Notification Config A
        config_recipient_row_repo
            .insert_one(&NotificationConfigRecipientRow {
                id: "1-id".to_string(),
                recipient_id: mock_recipient_a().id,
                notification_config_id: mock_coldchain_notification_config_a().id,
            })
            .unwrap();
        config_recipient_row_repo
            .insert_one(&NotificationConfigRecipientRow {
                id: "2-id".to_string(),
                recipient_id: mock_recipient_b().id,
                notification_config_id: mock_coldchain_notification_config_a().id,
            })
            .unwrap();

        // Update recipient ids to recipients A and AA (removing B and adding AA)
        let update_notification_config = context
            .service_provider
            .notification_config_service
            .update_notification_config(
                &context,
                UpdateNotificationConfig {
                    id: mock_coldchain_notification_config_a().id,
                    title: None,
                    configuration_data: None,
                    recipient_ids: Some(vec![mock_recipient_a().id, mock_recipient_aa().id]),
                    recipient_list_ids: None,
                },
            );

        assert!(update_notification_config.is_ok());

        let db_config_recipients = config_recipient_repo
            .query_by_filter(
                NotificationConfigRecipientFilter::new().notification_config_id(
                    EqualFilter::equal_to(&mock_coldchain_notification_config_a().id),
                ),
            )
            .unwrap();

        // 2 recipients for config A, recipient A and AA
        assert_eq!(db_config_recipients.len(), 2);
        assert!(db_config_recipients
            .iter()
            .any(|r| r.recipient_id == mock_recipient_a().id));
        assert!(db_config_recipients
            .iter()
            .any(|r| r.recipient_id == mock_recipient_aa().id));
    }

    #[actix_rt::test]
    async fn notification_config_service_update_recipient_list_ids() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_update_recipient_list_ids",
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

        // Add recipient_lists B and C to Notification Config A
        config_recipient_list_row_repo
            .insert_one(&NotificationConfigRecipientListRow {
                id: "1-id".to_string(),
                recipient_list_id: mock_recipient_list_b_with_no_members().id,
                notification_config_id: mock_coldchain_notification_config_a().id,
            })
            .unwrap();
        config_recipient_list_row_repo
            .insert_one(&NotificationConfigRecipientListRow {
                id: "2-id".to_string(),
                recipient_list_id: mock_recipient_list_c().id,
                notification_config_id: mock_coldchain_notification_config_a().id,
            })
            .unwrap();

        // Update recipient list ids to recipient lists C and C2 (removing B and adding C2)
        let update_notification_config = context
            .service_provider
            .notification_config_service
            .update_notification_config(
                &context,
                UpdateNotificationConfig {
                    id: mock_coldchain_notification_config_a().id,
                    title: None,
                    configuration_data: None,
                    recipient_ids: None,
                    recipient_list_ids: Some(vec![
                        mock_recipient_list_c().id,
                        mock_recipient_list_c2().id,
                    ]),
                },
            );

        assert!(update_notification_config.is_ok());

        let db_config_recipient_lists = config_recipient_list_repo
            .query_by_filter(
                NotificationConfigRecipientListFilter::new().notification_config_id(
                    EqualFilter::equal_to(&mock_coldchain_notification_config_a().id),
                ),
            )
            .unwrap();

        // 2 recipient lists for config A, list C and C2
        assert_eq!(db_config_recipient_lists.len(), 2);
        assert!(db_config_recipient_lists
            .iter()
            .any(|r| r.recipient_list_id == mock_recipient_list_c().id));
        assert!(db_config_recipient_lists
            .iter()
            .any(|r| r.recipient_list_id == mock_recipient_list_c2().id));
    }
}
