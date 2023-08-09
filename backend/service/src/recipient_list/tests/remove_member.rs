#[cfg(test)]
mod recipient_list_member_remove_test {
    use repository::{mock::MockDataInserts, test_db::setup_all};
    use repository::{EqualFilter, RecipientListMemberFilter, RecipientListMemberRepository};
    use std::sync::Arc;

    use crate::recipient_list::remove_member::RemoveRecipientFromList;
    use crate::recipient_list::ModifyRecipientListError;
    use crate::service_provider::ServiceContext;
    use crate::service_provider::ServiceProvider;

    use crate::test_utils::get_test_settings;
    #[actix_rt::test]
    async fn remove_recipient_from_list_service_errors() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "remove_recipient_from_list_service_errors",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::new(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        // Add removing recipient from list it is not a part of
        assert_eq!(
            service.remove_recipient_from_list(
                &context,
                RemoveRecipientFromList {
                    // No mock data set up, so recipent[0] not in recipient_list[0]
                    recipient_id: mock_data["base"].recipients[0].id.clone(),
                    recipient_list_id: mock_data["base"].recipient_lists[0].id.clone(),
                },
            ),
            Err(ModifyRecipientListError::RecipientListMemberDoesNotExist)
        );
    }

    #[actix_rt::test]
    async fn remove_recipient_from_list_service_success() {
        let (mock_data, _, connection_manager, _) = setup_all(
            "remove_recipient_from_list_service_success",
            MockDataInserts::none().recipient_list_members(),
        )
        .await;

        let connection = connection_manager.connection().unwrap();
        let recipient_list_member_repository = RecipientListMemberRepository::new(&connection);
        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let context = ServiceContext::as_server_admin(service_provider).unwrap();
        let service = &context.service_provider.recipient_list_service;

        let result = service.remove_recipient_from_list(
            &context,
            RemoveRecipientFromList {
                // as per mock data setup, recipient[0] is part of recipient_list[0]
                recipient_id: mock_data["base"].recipients[0].id.clone(),
                recipient_list_id: mock_data["base"].recipient_lists[0].id.clone(),
            },
        );

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        assert!(result.is_ok());

        assert_eq!(
            recipient_list_member_repository
                .query_by_filter(
                    RecipientListMemberFilter::new()
                        .recipient_id(EqualFilter::equal_to(&mock_data["base"].recipients[0].id))
                        .recipient_list_id(EqualFilter::equal_to(
                            &mock_data["base"].recipient_lists[0].id,
                        )),
                )
                .unwrap(),
            vec![]
        );
    }
}
