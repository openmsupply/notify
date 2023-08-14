#[cfg(test)]
mod recipient_update_tests {

    use std::sync::Arc;

    use repository::mock::{mock_recipient_a, mock_recipient_b};
    use repository::NotificationType;
    use repository::{mock::MockDataInserts, test_db::setup_all};

    use crate::recipient::create::CreateRecipient;
    use crate::recipient::update::UpdateRecipient;
    use crate::recipient::ModifyRecipientError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn recipient_service_update_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_update_errors",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        // Trying to updating Recipient that does not exist should fail
        assert_eq!(
            service.update_recipient(
                &context,
                UpdateRecipient {
                    id: "new_id".to_string(),
                    name: Some("new_name".to_string()),
                    to_address: None,
                },
            ),
            Err(ModifyRecipientError::RecipientDoesNotExist)
        );

        // Updating to a to_address that already exists should fail
        assert_eq!(
            service.update_recipient(
                &context,
                UpdateRecipient {
                    id: mock_recipient_a().id.clone(),
                    to_address: Some(mock_recipient_b().to_address.clone()),
                    name: None,
                },
            ),
            Err(ModifyRecipientError::RecipientAlreadyExists)
        );
    }
    #[actix_rt::test]
    async fn recipient_service_update_success() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_update_success",
            MockDataInserts::none().recipients().permissions(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();

        //Create a recipient to update
        context
            .service_provider
            .recipient_service
            .create_recipient(
                &context,
                CreateRecipient {
                    id: "id1".to_string(),
                    name: "new_recipient_1".to_string(),
                    to_address: "new_recipient_1@test.com".to_string(),
                    notification_type: NotificationType::Email,
                },
            )
            .unwrap();

        // Update name
        let updated_recipient = context
            .service_provider
            .recipient_service
            .update_recipient(
                &context,
                UpdateRecipient {
                    id: "id1".to_string(),
                    name: Some("name_for_id1".to_string()),
                    to_address: None,
                },
            )
            .unwrap();

        assert_eq!(updated_recipient.name, "name_for_id1".to_string());

        // Update email
        let updated_recipient = context
            .service_provider
            .recipient_service
            .update_recipient(
                &context,
                UpdateRecipient {
                    id: "id1".to_string(),
                    name: None,
                    to_address: Some("id1@example.com".to_string()),
                },
            )
            .unwrap();

        assert_eq!(updated_recipient.to_address, "id1@example.com".to_string());
    }
}
