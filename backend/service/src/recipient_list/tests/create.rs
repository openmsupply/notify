#[cfg(test)]
mod recipient_list_create_test {
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
        let (mock_data, _, connection_manager, _) = setup_all(
            "create_recipient_list_service_errors",
            MockDataInserts::all(),
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
                    id: mock_data["base"].recipient_lists[0].id.clone(),
                    name: mock_data["base"].recipient_lists[0].name.clone(),
                    description: mock_data["base"].recipient_lists[0].description.clone(),
                },
            ),
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn create_recipient_list_service_success() {
        let (_, _, connection_manager, _) = setup_all(
            "create_recipient_list_service_success",
            MockDataInserts::all(),
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
