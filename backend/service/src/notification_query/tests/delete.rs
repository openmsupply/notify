#[cfg(test)]
mod notification_query_delete_test {
    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, NotificationQueryFilter, NotificationQueryRepository, NotificationQueryRow,
        NotificationQueryRowRepository,
    };

    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::notification_query::delete::DeleteNotificationQueryError;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn notification_query_service_delete_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_query_service_delete_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        // NotificationQuery does not exist
        assert_eq!(
            service.delete_notification_query(&context, "invalid_id",),
            Err(DeleteNotificationQueryError::NotificationQueryDoesNotExist)
        );
    }
    #[actix_rt::test]
    async fn notification_query_service_delete_success() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_query_service_delete_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let notification_query_repository = NotificationQueryRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        let repo = NotificationQueryRowRepository::new(&connection);

        let id = "some-id".to_string();
        let notification_query = NotificationQueryRow {
            id: id.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        assert_eq!(
            service.delete_notification_query(&context, &id),
            Ok(id.clone())
        );

        assert_eq!(
            notification_query_repository
                .query_by_filter(NotificationQueryFilter::new().id(EqualFilter::equal_to(&id)))
                .unwrap(),
            vec![]
        );
    }
}
