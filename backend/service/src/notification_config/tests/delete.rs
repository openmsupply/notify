#[cfg(test)]
mod notification_config_delete_test {
    use std::sync::Arc;

    use repository::mock::mock_notification_config_b;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{EqualFilter, NotificationConfigFilter, NotificationConfigRepository};

    use crate::notification_config::delete::DeleteNotificationConfigError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn notification_config_service_delete_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_delete_errors",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // NotificationConfig does not exist
        assert_eq!(
            service.delete_notification_config(&context, "invalid_id",),
            Err(DeleteNotificationConfigError::NotificationConfigDoesNotExist)
        );
    }
    #[actix_rt::test]
    async fn notification_config_service_delete_success() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_config_service_delete_success",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let notification_config_repository = NotificationConfigRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        assert_eq!(
            service.delete_notification_config(&context, &mock_notification_config_b().id),
            Ok(mock_notification_config_b().id.clone())
        );

        assert_eq!(
            notification_config_repository
                .query_by_filter(
                    NotificationConfigFilter::new()
                        .id(EqualFilter::equal_to(&mock_notification_config_b().id))
                )
                .unwrap(),
            vec![]
        );
    }
}
