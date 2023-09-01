#[cfg(test)]
mod recipient_list_create_test {
    use repository::mock::mock_recipient_list_c;
    use repository::RecipientListRowRepository;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::recipient_list::create::CreateRecipientList;
    use crate::recipient_list::ModifyRecipientListError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn create_recipient_list_service_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "create_recipient_list_service_errors",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        //Create for a id that already exists (Should use update in this case)
        assert_eq!(
            service.create_recipient_list(
                &context,
                CreateRecipientList {
                    id: mock_recipient_list_c().id.clone(),
                    name: mock_recipient_list_c().name.clone(),
                    description: mock_recipient_list_c().description.clone(),
                    ..Default::default()
                },
            ),
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
        );

        // Create for a name that already exists
        assert_eq!(
            service.create_recipient_list(
                &context,
                CreateRecipientList {
                    id: "some-new-id".to_string(),
                    name: mock_recipient_list_c().name.clone(),
                    description: "nice new description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
        );

        // Create with an illegal name string
        assert_eq!(
            service.create_recipient_list(
                &context,
                CreateRecipientList {
                    id: "some-new-id".to_string(),
                    name: "name'; DROP TABLE Students;--".to_string(),
                    description: "some-new-description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
        );

        // Create with an inappropriate length name
        assert_eq!(
            service.create_recipient_list(
                &context,
                CreateRecipientList {
                    id: "some-new-id".to_string(),
                    // less than 3 chars when trimmed
                    name: "  x     ".to_string(),
                    description: "some-new-description".to_string(),
                    ..Default::default()
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
        );
        assert_eq!(
            service.create_recipient_list(
                &context,
                CreateRecipientList {
                    id: "some-new-id".to_string(),
                    name: "Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string(),
                    description: "some-new-description".to_string(),
                             sql_query: None,
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
        );
    }

    #[actix_rt::test]
    async fn create_recipient_list_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "create_recipient_list_service_success",
            MockDataInserts::none(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_list_row_repository = RecipientListRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        let new_recipient_list_id = uuid();
        let result = service.create_recipient_list(
            &context,
            CreateRecipientList {
                id: new_recipient_list_id.clone(),
                name: "new_recipient_list".to_string(),
                description: "This is a new recipient list".to_string(),
                sql_query: None,
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = recipient_list_row_repository
            .find_one_by_id(&new_recipient_list_id)
            .unwrap();

        // RecipientList now exists
        assert_eq!(result.unwrap().description, "This is a new recipient list");
    }
}
