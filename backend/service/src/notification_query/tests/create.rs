#[cfg(test)]
mod notification_query_create_test {

    use repository::NotificationQueryRowRepository;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::notification_query::create::CreateNotificationQuery;
    use crate::notification_query::ModifyNotificationQueryError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn create_notification_query_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "create_notification_query_service_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        // Create an notification_query to confirm duplicate checks work
        let existing_notification_query_id = uuid();
        let existing_notification_query_name = "SomeName".to_string();
        service
            .create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: existing_notification_query_id.clone(),
                    name: existing_notification_query_name.clone(),
                    ..Default::default()
                },
            )
            .unwrap();

        // Create for a id that already exists (Should use update in this case)
        assert_eq!(
            service.create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: existing_notification_query_id,
                    name: existing_notification_query_name.clone(),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists)
        );

        // Create for a name that already exists
        assert_eq!(
            service.create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: "some-new-id".to_string(),
                    name: existing_notification_query_name,
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists)
        );

        // Create with an illegal name string
        assert_eq!(
            service.create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: "some-new-id".to_string(),
                    name: "name'; DROP TABLE Students;--".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );

        // Create with an inappropriate length name
        assert_eq!(
            service.create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: "some-new-id".to_string(),
                    // less than 3 chars when trimmed
                    name: "  x     ".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );
        assert_eq!(
            service.create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: "some-new-id".to_string(),
                    name: "Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );
    }

    #[actix_rt::test]
    async fn create_notification_query_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "create_notification_query_service_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let notification_query_row_repository = NotificationQueryRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        let new_notification_query_id = uuid();
        let result = service.create_notification_query(
            &context,
            CreateNotificationQuery {
                id: new_notification_query_id.clone(),
                name: "new_notification_query".to_string(),
                ..Default::default()
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = notification_query_row_repository
            .find_one_by_id(&new_notification_query_id)
            .unwrap()
            .unwrap();

        // NotificationQuery now exists
        assert_eq!(result.name, "new_notification_query");
    }
}
