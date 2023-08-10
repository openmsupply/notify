#[cfg(test)]
mod recipient_list_update_tests {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};

    use crate::recipient_list::create::CreateRecipientList;
    use crate::recipient_list::update::UpdateRecipientList;
    use crate::recipient_list::ModifyRecipientListError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn recipient_list_service_update_errors() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "recipient_list_service_update_errors",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        // Trying to updating RecipientList that does not exist should fail
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: "new_id".to_string(),
                    name: Some("new_name".to_string()),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::RecipientListDoesNotExist)
        );

        // Trying to update to a name that already exists should fail
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: mock_data["base"].recipient_lists[0].id.clone(),
                    name: Some(mock_data["base"].recipient_lists[1].name.clone()),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
        );
    }
    #[actix_rt::test]
    async fn recipient_list_service_update_success() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_list_service_update_success",
            MockDataInserts::none().recipient_lists().permissions(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        //Create a recipient_list to update
        context
            .service_provider
            .recipient_list_service
            .create_recipient_list(
                &context,
                CreateRecipientList {
                    id: "id1".to_string(),
                    name: "new_recipient_list_1".to_string(),
                    description: "descrizzle".to_string(),
                },
            )
            .unwrap();

        // Update name
        let updated_recipient_list = context
            .service_provider
            .recipient_list_service
            .update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: "id1".to_string(),
                    name: Some("name_for_id1".to_string()),
                    description: None,
                },
            )
            .unwrap();

        // updated
        assert_eq!(updated_recipient_list.name, "name_for_id1".to_string());
        // unchanged
        assert_eq!(updated_recipient_list.description, "descrizzle".to_string());

        // Update description
        let updated_recipient_list = context
            .service_provider
            .recipient_list_service
            .update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: "id1".to_string(),
                    name: None,
                    description: Some("A nice new description".to_string()),
                },
            )
            .unwrap();

        assert_eq!(
            updated_recipient_list.description,
            "A nice new description".to_string()
        );
    }
}
