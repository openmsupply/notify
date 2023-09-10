#[cfg(test)]
mod sql_recipient_list_query_test {
    use std::sync::Arc;

    use repository::{
        mock::MockDataInserts, test_db::setup_all, SqlRecipientListFilter,
        SqlRecipientListSortField,
    };
    use repository::{Sort, SqlRecipientListRow, SqlRecipientListRowRepository, StringFilter};

    use crate::service_provider::ServiceContext;
    use crate::test_utils::get_test_settings;
    use crate::{service_provider::ServiceProvider, SingleRecordError};

    #[actix_rt::test]
    async fn sql_recipient_list_service_single_record() {
        let (_, _, connection_manager, _) = setup_all(
            "test_sql_recipient_list_single_record",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        assert_eq!(
            service.get_sql_recipient_list(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let repo = SqlRecipientListRowRepository::new(&context.connection);

        let id = "some-id".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        let db_sql_recipient_list = service
            .get_sql_recipient_list(&context, id.clone())
            .unwrap();

        assert_eq!(db_sql_recipient_list.id, id);
    }

    #[actix_rt::test]
    async fn sql_recipient_list_service_filter() {
        let (_, _, connection_manager, _) =
            setup_all("test_sql_recipient_list_filter", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        // Setup a test record
        let repo = SqlRecipientListRowRepository::new(&context.connection);

        let id = "some-id".to_string();
        let name = "some-name".to_string();
        let description = "some-description".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id.clone(),
            name: name.clone(),
            description: description.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        // Query to find the new record
        let db_sql_recipient_lists = service
            .get_sql_recipient_lists(
                &context,
                None,
                Some(SqlRecipientListFilter::new().name(StringFilter::equal_to(&name))),
                None,
            )
            .unwrap();

        assert_eq!(db_sql_recipient_lists.count, 1);
        assert_eq!(db_sql_recipient_lists.rows[0].id, id);
    }

    #[actix_rt::test]
    async fn sql_recipient_list_service_filter_search() {
        let (_, _, connection_manager, _) = setup_all(
            "test_sql_recipient_list_filter_search",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        // Setup a test records
        let repo = SqlRecipientListRowRepository::new(&context.connection);

        let id1 = "id1".to_string();
        let name1 = "List A".to_string();
        let description1 = "List A".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id1.clone(),
            name: name1.clone(),
            description: description1.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        let id2 = "id2".to_string();
        let name2 = "List B: Search String".to_string();
        let description2 = "List B".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id2.clone(),
            name: name2.clone(),
            description: description2.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        let id3 = "id3".to_string();
        let name3 = "List C".to_string();
        let description3 = "List C: Search String".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id3.clone(),
            name: name3.clone(),
            description: description3.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        // Query to find the new records

        let db_sql_recipient_lists = service
            .get_sql_recipient_lists(
                &context,
                None,
                Some(SqlRecipientListFilter::new().search("Search String".to_string())),
                Some(Sort {
                    key: SqlRecipientListSortField::Id,
                    desc: Some(false),
                }),
            )
            .unwrap();

        assert_eq!(db_sql_recipient_lists.count, 2);
        assert_eq!(db_sql_recipient_lists.rows[0].id, id2);
        assert_eq!(db_sql_recipient_lists.rows[1].id, id3);
    }
}
