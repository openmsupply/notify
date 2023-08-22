#[cfg(test)]
mod notification_config_query_test {
    use std::sync::Arc;

    use repository::mock::mock_notification_config_a;
    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationConfigFilter,
        NotificationConfigSortField,
    };
    use repository::{EqualFilter, PaginationOption, Sort};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, ListError, SingleRecordError};

    #[actix_rt::test]
    async fn notification_config_service_pagination() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_config_service_pagination",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        assert_eq!(
            service.get_notification_configs(
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
            service.get_notification_configs(
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
    async fn notification_config_service_single_record() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_config_single_record",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        assert_eq!(
            service.get_notification_config(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let db_notification_config = service
            .get_notification_config(&context, mock_notification_config_a().id.clone())
            .unwrap();

        assert_eq!(
            db_notification_config.id,
            mock_notification_config_a().id.clone()
        );
    }

    #[actix_rt::test]
    async fn notification_config_service_filter() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_config_filter",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let db_notification_configs = service
            .get_notification_configs(
                &context,
                None,
                Some(
                    NotificationConfigFilter::new()
                        .id(EqualFilter::equal_to(&mock_notification_config_a().id)),
                ),
                None,
            )
            .unwrap();

        assert_eq!(db_notification_configs.count, 1);
        assert_eq!(
            db_notification_configs.rows[0].id,
            mock_notification_config_a().id.clone()
        );
    }

    #[actix_rt::test]
    async fn notification_config_service_sort() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "test_notification_config_sort",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // Test name sort with default sort order
        let db_notification_configs = service
            .get_notification_configs(
                &context,
                None,
                None,
                Some(Sort {
                    key: NotificationConfigSortField::Title,
                    desc: None,
                }),
            )
            .unwrap();

        let mut notification_configs = mock_data["base"].notification_configs.clone();
        notification_configs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

        let db_names: Vec<String> = db_notification_configs
            .rows
            .into_iter()
            .map(|notification_config| notification_config.title)
            .collect();
        let sorted_names: Vec<String> = notification_configs
            .into_iter()
            .map(|notification_config| notification_config.title)
            .collect();

        assert_eq!(db_names, sorted_names);

        // Test Name sort with desc sort
        let db_notification_configs = service
            .get_notification_configs(
                &context,
                None,
                None,
                Some(Sort {
                    key: NotificationConfigSortField::Title,
                    desc: Some(true),
                }),
            )
            .unwrap();

        let mut notification_configs = mock_data["base"].notification_configs.clone();
        notification_configs.sort_by(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));

        let result_names: Vec<String> = db_notification_configs
            .rows
            .into_iter()
            .map(|notification_config| notification_config.title)
            .collect();
        let sorted_names: Vec<String> = notification_configs
            .into_iter()
            .map(|notification_config| notification_config.title)
            .collect();

        assert_eq!(result_names, sorted_names);
    }
}
