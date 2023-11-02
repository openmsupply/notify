#[cfg(test)]
mod notification_event_query_test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationEventFilter,
        NotificationEventSortField,
    };
    use repository::{NotificationEventRow, NotificationEventRowRepository, Sort};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, SingleRecordError};

    #[actix_rt::test]
    async fn notification_event_service_single_record() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_event_single_record",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_event_service;

        assert_eq!(
            service.get_notification_event(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let repo = NotificationEventRowRepository::new(&context.connection);

        let id = "some-id".to_string();
        let notification_event = NotificationEventRow {
            id: id.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        let db_notification_event = service
            .get_notification_event(&context, id.clone())
            .unwrap();

        assert_eq!(db_notification_event.id, id);
    }

    #[actix_rt::test]
    async fn notification_event_service_filter_search() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_event_filter_search",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_event_service;

        /* A search filter, should match strings in the title, message,to_address and error_message */

        // Setup test records
        let repo = NotificationEventRowRepository::new(&context.connection);

        let searched_string = "searched_string".to_string();

        let id0 = "id0-no-match".to_string();
        let notification_event = NotificationEventRow {
            id: id0.clone(),

            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        let id1 = "id1-title".to_string();
        let notification_event = NotificationEventRow {
            id: id1.clone(),
            title: Some(searched_string.clone()),
            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        let id2 = "id2-message".to_string();
        let notification_event = NotificationEventRow {
            id: id2.clone(),
            message: searched_string.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        let id3 = "id3-to_address".to_string();
        let notification_event = NotificationEventRow {
            id: id3.clone(),
            to_address: searched_string.clone(),
            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        let id4 = "id4-error-message".to_string();
        let notification_event = NotificationEventRow {
            id: id4.clone(),
            error_message: Some(searched_string.clone()),
            ..Default::default()
        };
        repo.insert_one(&notification_event).unwrap();

        // Query to find the new records
        let db_notification_events = service
            .get_notification_events(
                &context,
                None,
                Some(NotificationEventFilter::new().search(searched_string)),
                Some(Sort {
                    key: NotificationEventSortField::Id,
                    desc: Some(false),
                }),
            )
            .unwrap();

        assert_eq!(db_notification_events.count, 4);
        assert_eq!(db_notification_events.rows[0].id, id1);
        assert_eq!(db_notification_events.rows[1].id, id2);
        assert_eq!(db_notification_events.rows[2].id, id3);
        assert_eq!(db_notification_events.rows[3].id, id4);
    }
}
