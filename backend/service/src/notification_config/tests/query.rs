#[cfg(test)]
mod notification_config_query_test {
    use std::sync::Arc;

    use chrono::Utc;
    use repository::mock::mock_coldchain_notification_config_a;
    use repository::{
        mock::MockDataInserts, test_db::setup_all, NotificationConfigFilter,
        NotificationConfigSortField,
    };
    use repository::{
        EqualFilter, NotificationConfigRow, NotificationConfigRowRepository, PaginationOption, Sort, NotificationConfigStatus,
    };
    use util::uuid::uuid;

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
            .get_notification_config(&context, mock_coldchain_notification_config_a().id.clone())
            .unwrap();

        assert_eq!(
            db_notification_config.id,
            mock_coldchain_notification_config_a().id.clone()
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
                Some(NotificationConfigFilter::new().id(EqualFilter::equal_to(
                    &mock_coldchain_notification_config_a().id,
                ))),
                None,
            )
            .unwrap();

        assert_eq!(db_notification_configs.count, 1);
        assert_eq!(
            db_notification_configs.rows[0].id,
            mock_coldchain_notification_config_a().id.clone()
        );
    }

    #[actix_rt::test]
    async fn notification_config_service_search() {
        let (_, _, connection_manager, _) = setup_all(
            "test_notification_config_search",
            MockDataInserts::none().notification_configs(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        let repo = NotificationConfigRowRepository::new(&connection);

        let id_1 = uuid();
        let id_2 = uuid();
        let id_3 = uuid();

        // Insert 2 configs, 2 with "XXXX" as a substring and one without
        repo.insert_one(&NotificationConfigRow {
            id: id_1.clone(),
            title: "title with XXXX as substr".to_string(),
            ..Default::default()
        })
        .unwrap();
        repo.insert_one(&NotificationConfigRow {
            id: id_2.clone(),
            title: "XXXXXXXXX".to_string(),
            ..Default::default()
        })
        .unwrap();
        repo.insert_one(&NotificationConfigRow {
            id: id_3,
            title: "non-matching title".to_string(),
            ..Default::default()
        })
        .unwrap();

        let db_notification_configs = service
            .get_notification_configs(
                &context,
                None,
                Some(NotificationConfigFilter::new().search("XXXX".to_string())),
                Some(Sort {
                    key: NotificationConfigSortField::Title,
                    desc: None,
                }),
            )
            .unwrap();

        assert_eq!(db_notification_configs.count, 2);
        assert_eq!(db_notification_configs.rows[0].id, id_1);
        assert_eq!(db_notification_configs.rows[1].id, id_2);
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

    #[actix_rt::test]
    async fn test_get_notification_configs_by_kind_and_next() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "test_get_notification_configs_by_kind_and_next",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.notification_config_service;

        // Create a notification config
        let mut config = NotificationConfigRow {
            id: uuid(),
            title: "test".to_string(),
            kind: mock_data["base"].notification_configs[0].kind.clone(),
            configuration_data: "{}".to_string(),
            status: NotificationConfigStatus::Enabled,
            parameters: "{}".to_string(),
            ..Default::default()
        };

        let repo = NotificationConfigRowRepository::new(&context.connection);
        repo.insert_one(&config).unwrap();

        // Create a disabled notification config
        let config2 = NotificationConfigRow {
            id: uuid(),
            title: "test1".to_string(),
            kind: mock_data["base"].notification_configs[0].kind.clone(),
            configuration_data: "{}".to_string(),
            status: NotificationConfigStatus::Disabled,
            parameters: "{}".to_string(),
            ..Default::default()
        };
        repo.insert_one(&config2).unwrap();

        // Check that only the enabled config is returned by the due query
        let result = service
            .find_all_due_by_kind(&context, config.kind.clone(), Utc::now().naive_utc())
            .unwrap();

        assert_eq!(result.len(), 1);

        // set the last check date to now
        // and next check to 1 hour from now
        config.last_run_datetime = Some(Utc::now().naive_utc());
        config.next_due_datetime = Some(Utc::now().naive_utc() + chrono::Duration::hours(1));
        repo.update_one(&config).unwrap();

        // Check that it is not returned by the due query now
        let result = service
            .find_all_due_by_kind(&context, config.kind.clone(), Utc::now().naive_utc())
            .unwrap();

        assert_eq!(result.len(), 0);

        // Check that it is returned by the due query after 1 hour
        let result = service
            .find_all_due_by_kind(
                &context,
                config.kind.clone(),
                Utc::now().naive_utc() + chrono::Duration::hours(1),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
    }
}
