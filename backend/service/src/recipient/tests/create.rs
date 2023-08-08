#[cfg(test)]
mod recipient_create_test {
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{NotificationType, RecipientRowRepository};
    use std::sync::Arc;
    use util::uuid::uuid;

    use crate::recipient::create::CreateRecipient;
    use crate::recipient::ModifyRecipientError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn create_recipient_service_errors() {
        let (mock_data, _, connection_manager, _) =
            setup_all("create_recipient_service_errors", MockDataInserts::all()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        //Create for a id that already exists
        assert_eq!(
            service.create_recipient(
                &context,
                CreateRecipient {
                    id: mock_data["base"].recipients[0].id.clone(),
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
                    to_address: mock_data["base"].recipients[0].to_address.clone(),
                    notification_type: NotificationType::Email,
                },
            ),
            Err(ModifyRecipientError::RecipientAlreadyExists)
        );
    }

    #[actix_rt::test]
    async fn create_recipient_service_success() {
        let (_, _, connection_manager, _) =
            setup_all("create_recipient_service_success", MockDataInserts::all()).await;

        let connection = connection_manager.connection().unwrap();
        let recipient_row_repository = RecipientRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
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
}
