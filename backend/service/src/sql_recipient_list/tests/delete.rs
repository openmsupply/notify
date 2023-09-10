#[cfg(test)]
mod sql_recipient_list_delete_test {
    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, SqlRecipientListFilter, SqlRecipientListRepository, SqlRecipientListRow,
        SqlRecipientListRowRepository,
    };

    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::sql_recipient_list::delete::DeleteSqlRecipientListError;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn sql_recipient_list_service_delete_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "sql_recipient_list_service_delete_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        // SqlRecipientList does not exist
        assert_eq!(
            service.delete_sql_recipient_list(&context, "invalid_id",),
            Err(DeleteSqlRecipientListError::SqlRecipientListDoesNotExist)
        );
    }
    #[actix_rt::test]
    async fn sql_recipient_list_service_delete_success() {
        let (_, _, connection_manager, _) = setup_all(
            "sql_recipient_list_service_delete_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let sql_recipient_list_repository = SqlRecipientListRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        let repo = SqlRecipientListRowRepository::new(&connection);

        let id = "some-id".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        assert_eq!(
            service.delete_sql_recipient_list(&context, &id),
            Ok(id.clone())
        );

        assert_eq!(
            sql_recipient_list_repository
                .query_by_filter(SqlRecipientListFilter::new().id(EqualFilter::equal_to(&id)))
                .unwrap(),
            vec![]
        );
    }
}
