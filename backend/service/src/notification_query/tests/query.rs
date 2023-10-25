#[cfg(test)]
mod notification_query_query_test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationQueryFilter,
        NotificationQuerySortField,
    };
    use repository::{NotificationQueryRow, NotificationQueryRowRepository, Sort, StringFilter};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, SingleRecordError};

    #[actix_rt::test]
    async fn notification_query_service_single_record() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_query_single_record",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        assert_eq!(
            service.get_notification_query(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let repo = NotificationQueryRowRepository::new(&context.connection);

        let id = "some-id".to_string();
        let notification_query = NotificationQueryRow {
            id: id.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        let db_notification_query = service
            .get_notification_query(&context, id.clone())
            .unwrap();

        assert_eq!(db_notification_query.id, id);
    }

    #[actix_rt::test]
    async fn notification_query_service_filter() {
        let (_, _, connection_manager, _) =
            setup_all("test_notification_query_filter", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        // Setup a test record
        let repo = NotificationQueryRowRepository::new(&context.connection);

        let id = "some-id".to_string();
        let name = "some-name".to_string();
        let description = "some-description".to_string();
        let notification_query = NotificationQueryRow {
            id: id.clone(),
            name: name.clone(),
            description: description.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        // Query to find the new record
        let db_notification_queries = service
            .get_notification_queries(
                &context,
                None,
                Some(NotificationQueryFilter::new().name(StringFilter::equal_to(&name))),
                None,
            )
            .unwrap();

        assert_eq!(db_notification_queries.count, 1);
        assert_eq!(db_notification_queries.rows[0].id, id);
    }

    #[actix_rt::test]
    async fn notification_query_service_filter_search() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_query_filter_search",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_query_service;

        // Setup a test records
        let repo = NotificationQueryRowRepository::new(&context.connection);

        let id1 = "id1".to_string();
        let name1 = "List A".to_string();
        let description1 = "List A".to_string();
        let notification_query = NotificationQueryRow {
            id: id1.clone(),
            name: name1.clone(),
            description: description1.clone(),
            reference_name: "reference_name1".to_string(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        let id2 = "id2".to_string();
        let name2 = "List B: Search String".to_string();
        let description2 = "List B".to_string();
        let notification_query = NotificationQueryRow {
            id: id2.clone(),
            name: name2.clone(),
            description: description2.clone(),
            reference_name: "reference_name2".to_string(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        let id3 = "id3".to_string();
        let name3 = "List C".to_string();
        let description3 = "List C: Search String".to_string();
        let notification_query = NotificationQueryRow {
            id: id3.clone(),
            name: name3.clone(),
            description: description3.clone(),
            reference_name: "reference_name3".to_string(),
            ..Default::default()
        };
        repo.insert_one(&notification_query).unwrap();

        // Query to find the new records

        let db_notification_queries = service
            .get_notification_queries(
                &context,
                None,
                Some(NotificationQueryFilter::new().search("Search String".to_string())),
                Some(Sort {
                    key: NotificationQuerySortField::Id,
                    desc: Some(false),
                }),
            )
            .unwrap();

        assert_eq!(db_notification_queries.count, 2);
        assert_eq!(db_notification_queries.rows[0].id, id2);
        assert_eq!(db_notification_queries.rows[1].id, id3);
    }
}
