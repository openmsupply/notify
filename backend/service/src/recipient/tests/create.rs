#[cfg(test)]
mod recipient_create_test {
    use repository::mock::MockDataInserts;
    use repository::mock::{mock_recipient_a, mock_recipient_d_deleted};
    use repository::{NotificationType, RecipientRowRepository};
    use util::uuid::uuid;

    use crate::recipient::create::CreateRecipient;
    use crate::recipient::ModifyRecipientError;

    use crate::test_utils::get_test_service_context;
    #[actix_rt::test]
    async fn create_recipient_service_errors() {
        let (context, _) = get_test_service_context(MockDataInserts::none().recipients()).await;
        let service = &context.service_provider.recipient_service;

        //Create for a id that already exists
        assert_eq!(
            service.create_recipient(
                &context,
                CreateRecipient {
                    id: mock_recipient_a().id.clone(),
                    name: "some name".to_string(),
                    to_address: "some@address.com".to_string(),
                    notification_type: NotificationType::Email,
                },
            ),
            Err(ModifyRecipientError::RecipientAlreadyExists)
        );

        //Create for an id that already exists, but is soft deleted
        assert_eq!(
            service.create_recipient(
                &context,
                CreateRecipient {
                    id: mock_recipient_d_deleted().id.clone(),
                    name: "some name".to_string(),
                    to_address: "some@address.com".to_string(),
                    notification_type: NotificationType::Email,
                },
            ),
            Err(ModifyRecipientError::RecipientAlreadyExists)
        );

        //Create for a to_address that already exists
        assert_eq!(
            service.create_recipient(
                &context,
                CreateRecipient {
                    id: "some-new-id".to_string(),
                    name: "some name".to_string(),
                    to_address: mock_recipient_a().to_address.clone(),
                    notification_type: NotificationType::Email,
                },
            ),
            Err(ModifyRecipientError::RecipientAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn create_recipient_service_success() {
        let (context, connection) =
            get_test_service_context(MockDataInserts::none().recipients()).await;
        let recipient_row_repository = RecipientRowRepository::new(&connection);
        let service = &context.service_provider.recipient_service;

        let new_recipient_id = uuid();
        let result = service.create_recipient(
            &context,
            CreateRecipient {
                id: new_recipient_id.clone(),
                name: "new_recipient".to_string(),
                to_address: "New_recipient@test.com".to_string(),
                notification_type: NotificationType::Email,
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = recipient_row_repository
            .find_one_by_id(&new_recipient_id)
            .unwrap();

        // Recipient now exists
        assert!(result.is_some());

        // Recipient email lowercased correctly
        assert_eq!(result.unwrap().to_address, "new_recipient@test.com");
    }

    #[actix_rt::test]
    async fn create_recipient_same_to_address_different_type_success() {
        let (context, connection) =
            get_test_service_context(MockDataInserts::none().recipients()).await;
        let recipient_row_repository = RecipientRowRepository::new(&connection);
        let service = &context.service_provider.recipient_service;

        let new_recipient_id = uuid();
        let result = service.create_recipient(
            &context,
            CreateRecipient {
                id: new_recipient_id.clone(),
                name: "new_recipient".to_string(),
                to_address: mock_recipient_a().to_address.clone(),
                // mock_recipient_a is Email type, so same to_address for Telegram type should succeed
                notification_type: NotificationType::Telegram,
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        let result = recipient_row_repository
            .find_one_by_id(&new_recipient_id)
            .unwrap();

        // Recipient now exists
        assert!(result.is_some());
    }
}
