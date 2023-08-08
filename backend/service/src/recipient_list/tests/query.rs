#[cfg(test)]
mod recipient_list_query_test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, RecipientListFilter, RecipientListSortField,
    };
    use repository::{EqualFilter, PaginationOption, Sort};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, ListError, SingleRecordError};

    #[actix_rt::test]
    async fn recipient_list_service_pagination() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_list_service_pagination",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        assert_eq!(
            service.get_recipient_lists(
                &context,
                Some(PaginationOption {
                    limit: Some(2000),
                    offset: None
                }),
                None,
                None,
            ),
            Err(ListError::LimitAboveMax(1000))
        );

        assert_eq!(
            service.get_recipient_lists(
                &context,
                Some(PaginationOption {
                    limit: Some(0),
                    offset: None,
                }),
                None,
                None,
            ),
            Err(ListError::LimitBelowMin(1))
        );
    }

    #[actix_rt::test]
    async fn recipient_list_service_single_record() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_list_single_record",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        assert_eq!(
            service.get_recipient_list(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let db_recipient_list = service
            .get_recipient_list(&context, "id_recipient_list_a".to_owned())
            .unwrap();

        assert_eq!(db_recipient_list.id, "id_recipient_list_a");
    }

    #[actix_rt::test]
    async fn recipient_list_service_filter() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_list_filter",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        let db_recipient_lists = service
            .get_recipient_lists(
                &context,
                None,
                Some(RecipientListFilter::new().id(EqualFilter::equal_to("id_recipient_list_a"))),
                None,
            )
            .unwrap();

        assert_eq!(db_recipient_lists.count, 1);
        assert_eq!(db_recipient_lists.rows[0].id, "id_recipient_list_a");
    }

    #[actix_rt::test]
    async fn recipient_list_service_sort() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "test_recipient_list_sort",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        // Test name sort with default sort order
        let db_recipient_lists = service
            .get_recipient_lists(
                &context,
                None,
                None,
                Some(Sort {
                    key: RecipientListSortField::Name,
                    desc: None,
                }),
            )
            .unwrap();

        let mut recipient_lists = mock_data["base"].recipient_lists.clone();
        recipient_lists.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let db_names: Vec<String> = db_recipient_lists
            .rows
            .into_iter()
            .map(|recipient_list| recipient_list.name)
            .collect();
        let sorted_names: Vec<String> = recipient_lists
            .into_iter()
            .map(|recipient_list| recipient_list.name)
            .collect();

        assert_eq!(db_names, sorted_names);

        // Test Name sort with desc sort
        let db_recipient_lists = service
            .get_recipient_lists(
                &context,
                None,
                None,
                Some(Sort {
                    key: RecipientListSortField::Name,
                    desc: Some(true),
                }),
            )
            .unwrap();

        let mut recipient_lists = mock_data["base"].recipient_lists.clone();
        recipient_lists.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));

        let result_names: Vec<String> = db_recipient_lists
            .rows
            .into_iter()
            .map(|recipient_list| recipient_list.name)
            .collect();
        let sorted_names: Vec<String> = recipient_lists
            .into_iter()
            .map(|recipient_list| recipient_list.name)
            .collect();

        assert_eq!(result_names, sorted_names);
    }
}
