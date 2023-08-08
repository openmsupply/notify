use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::delete::DeleteRecipientListError as ServiceError,
};

pub fn delete_recipient_list(
    ctx: &Context<'_>,
    recipient_list_id: &str,
) -> Result<DeleteRecipientListResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::MutateRecipientLists,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    match service_context
        .service_provider
        .recipient_list_service
        .delete_recipient_list(&service_context, recipient_list_id)
    {
        Ok(recipient_list_id) => Ok(DeleteRecipientListResponse::Response(DeleteResponse(
            recipient_list_id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteRecipientListResponse {
    Response(DeleteResponse),
}

fn map_error(error: ServiceError) -> Result<DeleteRecipientListResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Structured Errors
        // Standard Graphql Errors
        ServiceError::RecipientListDoesNotExist => BadUserInput(formatted_error),
        ServiceError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use graphql_core::{
        assert_graphql_query, assert_standard_graphql_error, test_helpers::setup_graphql_test,
    };
    use repository::{mock::MockDataInserts, StorageConnectionManager};
    use serde_json::json;

    use service::{
        recipient_list::{delete::DeleteRecipientListError, RecipientListServiceTrait},
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientListMutations;

    type DeleteRecipientListMethod =
        dyn Fn(&str) -> Result<String, DeleteRecipientListError> + Sync + Send;

    pub struct TestService(pub Box<DeleteRecipientListMethod>);

    impl RecipientListServiceTrait for TestService {
        fn delete_recipient_list(
            &self,
            _: &ServiceContext,
            recipient_list_id: &str,
        ) -> Result<String, DeleteRecipientListError> {
            (self.0)(recipient_list_id)
        }
    }

    pub fn service_provider(
        recipient_list_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider =
            ServiceProvider::new(connection_manager.clone(), get_test_settings(""));
        service_provider.recipient_list_service = Box::new(recipient_list_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_delete_recipient_list_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_delete_recipient_list_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: String!) {
            deleteRecipientList(recipientListId: $input) {
            ... on DeleteResponse {
                id
                }
            }
          }
        "#;

        let variables = Some(json!({
          "input": "invalid_recipient_list_id"
        }));
        // Record Not Found
        let test_service = TestService(Box::new(|_| {
            Err(DeleteRecipientListError::RecipientListDoesNotExist)
        }));
        let expected_message = "Bad user input";

        assert_standard_graphql_error!(
            &settings,
            mutation,
            &variables,
            &expected_message,
            None,
            Some(service_provider(test_service, &connection_manager))
        );
    }

    #[actix_rt::test]
    async fn test_graphql_delete_recipient_list_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_delete_recipient_list_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: String!) {
            deleteRecipientList(recipientListId: $input) {
              ... on DeleteResponse {
                id
              }
            }
          }
        "#;

        let variables = Some(json!({
          "input": "recipient_list_id"
        }));

        let test_service = TestService(Box::new(|_| Ok("recipient_list_id".to_owned())));

        let expected = json!({
            "deleteRecipientList": {
                "id": "recipient_list_id",
            }
          }
        );

        assert_graphql_query!(
            &settings,
            mutation,
            &variables,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }
}
