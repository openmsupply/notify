#[cfg(test)]
mod sql_recipient_list_update_tests {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{SqlRecipientListRow, SqlRecipientListRowRepository};

    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::sql_recipient_list::create::CreateSqlRecipientList;
    use crate::sql_recipient_list::update::UpdateSqlRecipientList;
    use crate::sql_recipient_list::ModifySqlRecipientListError;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn sql_recipient_list_service_update_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "sql_recipient_list_service_update_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        // Setup test records
        let repo = SqlRecipientListRowRepository::new(&context.connection);
        let id1 = "id1".to_string();
        let name1 = "name1".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id1.clone(),
            name: name1.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        let id2 = "id2".to_string();
        let name2 = "name2".to_string();
        let sql_recipient_list = SqlRecipientListRow {
            id: id2.clone(),
            name: name2.clone(),
            ..Default::default()
        };
        repo.insert_one(&sql_recipient_list).unwrap();

        // Trying to updating SqlRecipientList that does not exist should fail
        assert_eq!(
            service.update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: "new_id".to_string(),
                    name: Some("new_name".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::SqlRecipientListDoesNotExist)
        );

        // Trying to update to a name that already exists should fail (even with added whitespace)
        assert_eq!(
            service.update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: id1.clone(),
                    name: Some(name2.clone() + "  "),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists)
        );

        // Trying to update to a name with illegal characters should fail
        assert_eq!(
            service.update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: id1.clone(),
                    name: Some("name'; DROP TABLE Students;--".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );

        // Trying to update to an inappropriate length of name should fail
        assert_eq!(
            service.update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: id1.clone(),
                    // less than 3 chars
                    name: Some("x".to_string()),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );
        assert_eq!(
            service.update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: id1.clone(),
                    name: Some("Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string()),
                           ..Default::default()

                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );
    }
    #[actix_rt::test]
    async fn sql_recipient_list_service_update_success() {
        let (_, _, connection_manager, _) = setup_all(
            "sql_recipient_list_service_update_success",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        //Create a sql_recipient_list to update
        context
            .service_provider
            .sql_recipient_list_service
            .create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: "id1".to_string(),
                    name: "new_sql_recipient_list_1".to_string(),
                    description: "descrizzle".to_string(),
                    ..Default::default()
                },
            )
            .unwrap();

        // Update name
        let updated_sql_recipient_list = context
            .service_provider
            .sql_recipient_list_service
            .update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: "id1".to_string(),
                    name: Some("name_for_id1".to_string()),
                    description: None,
                    ..Default::default()
                },
            )
            .unwrap();

        // updated
        assert_eq!(updated_sql_recipient_list.name, "name_for_id1".to_string());
        // unchanged
        assert_eq!(
            updated_sql_recipient_list.description,
            "descrizzle".to_string()
        );

        // Update description
        let updated_sql_recipient_list = context
            .service_provider
            .sql_recipient_list_service
            .update_sql_recipient_list(
                &context,
                UpdateSqlRecipientList {
                    id: "id1".to_string(),
                    name: None,
                    description: Some("A nice new description".to_string()),
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!(
            updated_sql_recipient_list.description,
            "A nice new description".to_string()
        );
    }
}
