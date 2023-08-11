#[cfg(test)]
mod recipient_list_member_remove_test {
    use repository::mock::{
        mock_recipient_a, mock_recipient_list_with_no_members,
        mock_recipient_list_with_recipient_members_a_and_b,
    };
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
        let (_, _, connection_manager, _) = setup_all(
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

        // removing recipient from list it is not a part of
        assert_eq!(
            service.remove_recipient_from_list(
                &context,
                RemoveRecipientFromList {
                    recipient_id: mock_recipient_a().id.clone(),
                    recipient_list_id: mock_recipient_list_with_no_members().id.clone(),
                },
            ),
            Err(ModifyRecipientListError::RecipientListMemberDoesNotExist)
        );
    }

    #[actix_rt::test]
    async fn remove_recipient_from_list_service_success() {
        let (_, _, connection_manager, _) = setup_all(
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
                recipient_id: mock_recipient_a().id.clone(),
                recipient_list_id: mock_recipient_list_with_recipient_members_a_and_b()
                    .id
                    .clone(),
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
                        .recipient_id(EqualFilter::equal_to(&mock_recipient_a().id))
                        .recipient_list_id(EqualFilter::equal_to(
                            &mock_recipient_list_with_recipient_members_a_and_b().id,
                        )),
                )
                .unwrap(),
            vec![]
        );
    }
}
