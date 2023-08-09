#[cfg(test)]
mod recipient_query_test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, RecipientFilter, RecipientSortField,
    };
    use repository::{EqualFilter, PaginationOption, Sort};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, ListError, SingleRecordError};

    #[actix_rt::test]
    async fn recipient_service_pagination() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_service_pagination",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
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
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_single_record",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            service.get_recipient(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let db_recipient = service
            .get_recipient(&context, "id_recipient_a".to_owned())
            .unwrap();

        assert_eq!(db_recipient.id, "id_recipient_a");
    }

    #[actix_rt::test]
    async fn recipient_service_filter() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_filter",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        let db_recipients = service
            .get_recipients(
                &context,
                None,
                Some(RecipientFilter::new().id(EqualFilter::equal_to("id_recipient_a"))),
                None,
            )
            .unwrap();

        assert_eq!(db_recipients.count, 1);
        assert_eq!(db_recipients.rows[0].id, "id_recipient_a");
    }

    #[actix_rt::test]
    async fn recipient_service_filter_search() {
        let (_, _, connection_manager, _) = setup_all(
            "test_recipient_filter_search",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
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
            "a@openmsupply.foundation".to_string()
        );
        assert_eq!(
            to_address_search_db_recipients.rows[1].to_address,
            "aa@openmsupply.foundation".to_string()
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
            "recipient_a".to_string()
        );
        assert_eq!(
            name_search_db_recipients.rows[1].name,
            "recipient_aa".to_string()
        );
    }

    #[actix_rt::test]
    async fn recipient_service_sort() {
        let (mock_data, _, connection_manager, _) =
            setup_all("test_recipient_sort", MockDataInserts::none().recipients()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

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

        let mut recipients = mock_data["base"].recipients.clone();
        recipients.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let db_names: Vec<String> = db_recipients
            .rows
            .into_iter()
            .map(|recipient| recipient.name)
            .collect();
        let sorted_names: Vec<String> = recipients
            .into_iter()
            .map(|recipient| recipient.name)
            .collect();

        assert_eq!(db_names, sorted_names);

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

        let mut recipients = mock_data["base"].recipients.clone();
        recipients.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));

        let result_names: Vec<String> = db_recipients
            .rows
            .into_iter()
            .map(|recipient| recipient.name)
            .collect();
        let sorted_names: Vec<String> = recipients
            .into_iter()
            .map(|recipient| recipient.name)
            .collect();

        assert_eq!(result_names, sorted_names);
    }
}
