#[cfg(test)]
mod recipient_delete_test {
    use std::sync::Arc;

    use repository::mock::{mock_recipient_a, mock_recipient_b};
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{
        EqualFilter, NotificationType, RecipientFilter, RecipientRepository, RecipientRowRepository,
    };
    use util::uuid::uuid;

    use crate::recipient::create::CreateRecipient;
    use crate::recipient::delete::DeleteRecipientError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;
    use crate::test_utils::get_test_settings;

    #[actix_rt::test]
    async fn recipient_service_delete_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_delete_errors",
            MockDataInserts::none().recipients(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        // Recipient does not exist
        assert_eq!(
            service.delete_recipient(&context, "invalid_id",),
            Err(DeleteRecipientError::RecipientDoesNotExist)
        );
    }
    #[actix_rt::test]
    async fn recipient_service_delete_success() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_delete_success",
            MockDataInserts::none().recipients(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_repository = RecipientRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            service.delete_recipient(&context, &mock_recipient_b().id),
            Ok(mock_recipient_b().id.clone())
        );

        assert_eq!(
            recipient_repository
                .query_by_filter(
                    RecipientFilter::new().id(EqualFilter::equal_to(&mock_recipient_b().id))
                )
                .unwrap(),
            vec![]
        );
    }

    #[actix_rt::test]
    async fn recipient_service_delete_when_is_list_member_success() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_delete_when_is_list_member_success",
            MockDataInserts::none().recipient_list_members(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_repository = RecipientRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            // mock_recipient_a is a part of recipient_list_a (which would create a FK constraint)
            service.delete_recipient(&context, &mock_recipient_a().id),
            Ok(mock_recipient_a().id.clone())
        );

        assert_eq!(
            recipient_repository
                .query_by_filter(
                    RecipientFilter::new().id(EqualFilter::equal_to(&mock_recipient_a().id))
                )
                .unwrap(),
            vec![]
        );
    }

    #[actix_rt::test]
    async fn recipient_service_delete_and_recreate() {
        let (_, _, connection_manager, _) = setup_all(
            "recipient_service_delete_and_recreate",
            MockDataInserts::none().recipient_list_members(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_row_repository = RecipientRowRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_service;

        assert_eq!(
            service.delete_recipient(&context, &mock_recipient_a().id),
            Ok(mock_recipient_a().id.clone())
        );

        let new_id = uuid();
        let recreate_result = service.create_recipient(
            &context,
            CreateRecipient {
                id: new_id.clone(),
                name: "recreated recipient A".to_string(),
                to_address: mock_recipient_a().to_address.clone(),
                notification_type: NotificationType::Email,
            },
        );

        if !recreate_result.is_ok() {
            println!("Error: {:?}", recreate_result);
        }
        assert!(recreate_result.is_ok());

        let new_recipient = recipient_row_repository.find_one_by_id(&new_id).unwrap();

        // Recipient now exists
        assert!(new_recipient.is_some());
    }
}
