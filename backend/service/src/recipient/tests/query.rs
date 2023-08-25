#[cfg(test)]
mod recipient_query_test {
    use repository::mock::{
        mock_recipient_a, mock_recipient_aa, mock_recipient_d_deleted, mock_recipients,
    };
    use repository::{mock::MockDataInserts, RecipientFilter, RecipientRow, RecipientSortField};
    use repository::{EqualFilter, PaginationOption, Sort};

    use crate::test_utils::get_test_service_context;
    use crate::{ListError, SingleRecordError};

    #[actix_rt::test]
    async fn recipient_service_pagination() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            service.get_recipients(
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
            service.get_recipients(
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
    async fn recipient_service_single_record() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            service.get_recipient(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let db_recipient = service
            .get_recipient(&context, mock_recipient_a().id.clone())
            .unwrap();

        assert_eq!(db_recipient.id, mock_recipient_a().id.clone());
    }

    #[actix_rt::test]
    async fn recipient_service_filter() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        let db_recipients = service
            .get_recipients(
                &context,
                None,
                Some(RecipientFilter::new().id(EqualFilter::equal_to(&mock_recipient_a().id))),
                None,
            )
            .unwrap();

        assert_eq!(db_recipients.count, 1);
        assert_eq!(db_recipients.rows[0].id, mock_recipient_a().id.clone());
    }

    #[actix_rt::test]
    async fn recipient_service_filters_deleted() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        let db_recipients = service
            .get_recipients(
                &context,
                None,
                Some(
                    RecipientFilter::new()
                        .id(EqualFilter::equal_to(&mock_recipient_d_deleted().id)),
                ),
                None,
            )
            .unwrap();

        assert_eq!(db_recipients.count, 0);
    }

    #[actix_rt::test]
    async fn recipient_service_filter_search() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        let to_address_search_db_recipients = service
            .get_recipients(
                &context,
                None,
                Some(RecipientFilter {
                    search: Some("a@openmsupply".to_string()),
                    ..Default::default()
                }),
                None,
            )
            .unwrap();

        assert_eq!(to_address_search_db_recipients.count, 2);
        assert_eq!(
            to_address_search_db_recipients.rows[0].to_address,
            mock_recipient_a().to_address.clone()
        );
        assert_eq!(
            to_address_search_db_recipients.rows[1].to_address,
            mock_recipient_aa().to_address.clone()
        );

        let name_search_db_recipients = service
            .get_recipients(
                &context,
                None,
                Some(RecipientFilter {
                    search: Some("recipient_a".to_string()),
                    ..Default::default()
                }),
                None,
            )
            .unwrap();

        assert_eq!(name_search_db_recipients.count, 2);
        assert_eq!(
            name_search_db_recipients.rows[0].name,
            mock_recipient_a().name.clone()
        );
        assert_eq!(
            name_search_db_recipients.rows[1].name,
            mock_recipient_aa().name.clone()
        );
    }

    #[actix_rt::test]
    async fn recipient_service_sort() {
        let context = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        let mock_data_names: Vec<String> = mock_recipients()
            .clone()
            .into_iter()
            .filter(|recipient| recipient.deleted_datetime.is_none())
            .map(|recipient| recipient.name)
            .collect();

        // Test name sort with default sort order
        let db_recipients = service
            .get_recipients(
                &context,
                None,
                None,
                Some(Sort {
                    key: RecipientSortField::Name,
                    desc: None,
                }),
            )
            .unwrap();

        let mut sorted_mock_names = mock_data_names.clone();
        sorted_mock_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        let db_names: Vec<String> = collect_names(db_recipients.rows);

        assert_eq!(db_names, sorted_mock_names);

        // Test Name sort with desc sort
        let db_recipients = service
            .get_recipients(
                &context,
                None,
                None,
                Some(Sort {
                    key: RecipientSortField::Name,
                    desc: Some(true),
                }),
            )
            .unwrap();

        let mut sorted_mock_names = mock_data_names.clone();
        sorted_mock_names.sort_by(|a, b| b.to_lowercase().cmp(&a.to_lowercase()));

        let result_names: Vec<String> = collect_names(db_recipients.rows);

        assert_eq!(result_names, sorted_mock_names);
    }

    fn collect_names(input: Vec<RecipientRow>) -> Vec<String> {
        input.into_iter().map(|recipient| recipient.name).collect()
    }
}
