#[cfg(test)]
mod recipient_list_member_add_test {
    use repository::RecipientListMemberRowRepository;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::recipient_list::add_member::AddRecipientToList;
    use crate::recipient_list::ModifyRecipientListError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn add_recipient_to_list_service_errors() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "add_recipient_to_list_service_errors",
            MockDataInserts::none().recipient_list_members(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        // Add recipient that doesn't exist
        assert_eq!(
            service.add_recipient_to_list(
                &context,
                AddRecipientToList {
                    id: "a good new id".to_string(),
                    recipient_id: "some-unknown-id".to_string(),
                    recipient_list_id: mock_data["base"].recipient_lists[0].id.clone(),
                },
            ),
            Err(ModifyRecipientListError::RecipientDoesNotExist)
        );

        // Add recipient to list that doesn't exist
        assert_eq!(
            service.add_recipient_to_list(
                &context,
                AddRecipientToList {
                    id: "a good new id".to_string(),
                    recipient_id: mock_data["base"].recipients[0].id.clone(),
                    recipient_list_id: "some-unknown-id".to_string(),
                },
            ),
            Err(ModifyRecipientListError::RecipientListDoesNotExist)
        );

        // Add recipient to list it is already a part of
        assert_eq!(
            service.add_recipient_to_list(
                &context,
                AddRecipientToList {
                    id: "some-new-id".to_string(),
                    recipient_id: mock_data["base"].recipient_list_members[0]
                        .recipient_id
                        .clone(),
                    recipient_list_id: mock_data["base"].recipient_list_members[0]
                        .recipient_list_id
                        .clone(),
                },
            ),
            Err(ModifyRecipientListError::RecipientListMemberAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn add_recipient_to_list_service_success() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "add_recipient_to_list_service_success",
            MockDataInserts::none().recipients().recipient_lists(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_list_member_row_repository =
            RecipientListMemberRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        let new_recipient_list_member_id = uuid();
        let result = service.add_recipient_to_list(
            &context,
            AddRecipientToList {
                id: new_recipient_list_member_id.clone(),
                recipient_id: mock_data["base"].recipients[0].id.clone(),
                recipient_list_id: mock_data["base"].recipient_lists[0].id.clone(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = recipient_list_member_row_repository
            .find_one_by_id(&new_recipient_list_member_id)
            .unwrap();

        // RecipientListMember now exists
        assert_eq!(
            result.unwrap().recipient_id,
            mock_data["base"].recipients[0].id.clone()
        );
    }
}
