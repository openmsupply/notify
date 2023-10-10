#[cfg(test)]
mod notification_query_update_tests {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{NotificationQueryRow, NotificationQueryRowRepository};

    use crate::notification_query::create::CreateNotificationQuery;
    use crate::notification_query::update::UpdateNotificationQuery;
    use crate::notification_query::ModifyNotificationQueryError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn notification_query_service_update_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_query_service_update_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        // Setup test records
        let repo = NotificationQueryRowRepository::new(&context.connection);
        let id1 = "id1".to_string();
        let name1 = "name1".to_string();
        let notification_query = NotificationQueryRow {
            id: id1.clone(),
            name: name1.clone(),
            reference_name: "reference_name1".to_string(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        let id2 = "id2".to_string();
        let name2 = "name2".to_string();
        let notification_query = NotificationQueryRow {
            id: id2.clone(),
            name: name2.clone(),
            reference_name: "reference_name2".to_string(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        // Trying to updating NotificationQuery that does not exist should fail
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: "new_id".to_string(),
                    name: Some("new_name".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::NotificationQueryDoesNotExist)
        );

        // Trying to update to a name that already exists should fail (even with added whitespace)
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: id1.clone(),
                    name: Some(name2.clone() + "  "),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists)
        );

        // Trying to update to a reference_name that already exists should fail (even with added whitespace)
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: id1.clone(),
                    reference_name: Some("reference_name2  ".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::ReferenceNameAlreadyExists)
        );

        // Trying to update to a name with illegal characters should fail
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: id1.clone(),
                    name: Some("name1!".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );

        // Trying to update to an inappropriate length of name should fail
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: id1.clone(),
                    // less than 3 chars
                    name: Some("x".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );
        assert_eq!(
            service.update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: id1.clone(),
                    name: Some("Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string()),
                           ..Default::default()

                },
            ),
            Err(ModifyNotificationQueryError::InvalidNotificationQueryName)
        );
    }
    #[actix_rt::test]
    async fn notification_query_service_update_success() {
        let (_, _, connection_manager, _) = setup_all(
            "notification_query_service_update_success",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        // Create a notification_query to update
        context
            .service_provider
            .notification_query_service
            .create_notification_query(
                &context,
                CreateNotificationQuery {
                    id: "id1".to_string(),
                    name: "new_notification_query_1".to_string(),
                    ..Default::default()
                },
            )
            .unwrap();

        // Update name
        let updated_notification_query = context
            .service_provider
            .notification_query_service
            .update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: "id1".to_string(),
                    name: Some("name_for_id1".to_string()),
                    description: None,
                    ..Default::default()
                },
            )
            .unwrap();

        // updated
        assert_eq!(updated_notification_query.name, "name_for_id1".to_string());
        // unchanged
        assert_eq!(updated_notification_query.description, "".to_string());

        // Update description
        let updated_notification_query = context
            .service_provider
            .notification_query_service
            .update_notification_query(
                &context,
                UpdateNotificationQuery {
                    id: "id1".to_string(),
                    name: None,
                    description: Some("A nice new description".to_string()),
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!(
            updated_notification_query.description,
            "A nice new description".to_string()
        );
    }
}
