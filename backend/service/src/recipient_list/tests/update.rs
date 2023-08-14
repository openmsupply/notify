#[cfg(test)]
mod recipient_list_update_tests {

    use std::sync::Arc;

    use repository::mock::{mock_recipient_list_c, mock_recipient_list_with_no_members};
    use repository::{mock::MockDataInserts, test_db::setup_all};

    use crate::recipient_list::create::CreateRecipientList;
    use crate::recipient_list::update::UpdateRecipientList;
    use crate::recipient_list::ModifyRecipientListError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn recipient_list_service_update_errors() {
        let (_, _, connection_manager, _) = setup_all(
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

        // Trying to update to a name that already exists should fail (even with added whitespace)
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: mock_recipient_list_with_no_members().id.clone(),
                    name: Some(mock_recipient_list_c().name.clone() + "  "),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
        );

        // Trying to update to a name with illegal characters should fail
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: mock_recipient_list_c().id.clone(),
                    name: Some("name'; DROP TABLE Students;--".to_string()),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
        );

        // Trying to update to an inappropriate length of name should fail
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: mock_recipient_list_c().id.clone(),
                    // less than 3 chars
                    name: Some("x".to_string()),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
        );
        assert_eq!(
            service.update_recipient_list(
                &context,
                UpdateRecipientList {
                    id: mock_recipient_list_c().id.clone(),
                    name: Some("Why hello there this is an exceedingly large recipient list name that really isn't necessary given you can provide a description :)".to_string()),
                    description: None,
                },
            ),
            Err(ModifyRecipientListError::InvalidRecipientListName)
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
