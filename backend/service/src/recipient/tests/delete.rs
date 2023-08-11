#[cfg(test)]
mod recipient_delete_test {
    use std::sync::Arc;

    use repository::mock::mock_recipient_b;
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{EqualFilter, RecipientFilter, RecipientRepository};

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
}
