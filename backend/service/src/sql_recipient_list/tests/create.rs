#[cfg(test)]
mod sql_recipient_list_create_test {

    use repository::SqlRecipientListRowRepository;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::sql_recipient_list::create::CreateSqlRecipientList;
    use crate::sql_recipient_list::ModifySqlRecipientListError;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn create_sql_recipient_list_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "create_sql_recipient_list_service_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        // Create an sql_recipient_list to confirm duplicate checks work
        let existing_sql_recipient_list_id = uuid();
        let existing_sql_recipient_list_name = "SomeName".to_string();
        service
            .create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: existing_sql_recipient_list_id.clone(),
                    name: existing_sql_recipient_list_name.clone(),
                    ..Default::default()
                },
            )
            .unwrap();

        // Create for a id that already exists (Should use update in this case)
        assert_eq!(
            service.create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: existing_sql_recipient_list_id,
                    name: existing_sql_recipient_list_name.clone(),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists)
        );

        // Create for a name that already exists
        assert_eq!(
            service.create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: "some-new-id".to_string(),
                    name: existing_sql_recipient_list_name,
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists)
        );

        // Create with an illegal name string
        assert_eq!(
            service.create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: "some-new-id".to_string(),
                    name: "name'; DROP TABLE Students;--".to_string(),
                    description: "some-new-description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );

        // Create with an inappropriate length name
        assert_eq!(
            service.create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: "some-new-id".to_string(),
                    // less than 3 chars when trimmed
                    name: "  x     ".to_string(),
                    description: "some-new-description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );
        assert_eq!(
            service.create_sql_recipient_list(
                &context,
                CreateSqlRecipientList {
                    id: "some-new-id".to_string(),
                    name: "Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string(),
                    description: "some-new-description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifySqlRecipientListError::InvalidSqlRecipientListName)
        );
    }

    #[actix_rt::test]
    async fn create_sql_recipient_list_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "create_sql_recipient_list_service_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let sql_recipient_list_row_repository = SqlRecipientListRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.sql_recipient_list_service;

        let new_sql_recipient_list_id = uuid();
        let result = service.create_sql_recipient_list(
            &context,
            CreateSqlRecipientList {
                id: new_sql_recipient_list_id.clone(),
                name: "new_sql_recipient_list".to_string(),
                description: "This is a new recipient list".to_string(),
                query: "SELECT * FROM users".to_string(),
                ..Default::default()
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = sql_recipient_list_row_repository
            .find_one_by_id(&new_sql_recipient_list_id)
            .unwrap()
            .unwrap();

        // SqlRecipientList now exists
        assert_eq!(result.description, "This is a new recipient list");
        assert_eq!(result.query, "SELECT * FROM users");
    }
}
