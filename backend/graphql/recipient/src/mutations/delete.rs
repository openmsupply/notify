use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient::delete::DeleteRecipientError as ServiceError,
};

pub fn delete_recipient(ctx: &Context<'_>, recipient_id: &str) -> Result<DeleteRecipientResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    match service_context
        .service_provider
        .recipient_service
        .delete_recipient(&service_context, recipient_id)
    {
        Ok(recipient_id) => Ok(DeleteRecipientResponse::Response(DeleteResponse(
            recipient_id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteRecipientResponse {
    Response(DeleteResponse),
}

fn map_error(error: ServiceError) -> Result<DeleteRecipientResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Structured Errors
        // Standard Graphql Errors
        ServiceError::RecipientDoesNotExist => BadUserInput(formatted_error),
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
        recipient::{delete::DeleteRecipientError, RecipientServiceTrait},
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientMutations;

    type DeleteRecipientMethod = dyn Fn(&str) -> Result<String, DeleteRecipientError> + Sync + Send;

    pub struct TestService(pub Box<DeleteRecipientMethod>);

    impl RecipientServiceTrait for TestService {
        fn delete_recipient(
            &self,
            _: &ServiceContext,
            recipient_id: &str,
        ) -> Result<String, DeleteRecipientError> {
            (self.0)(recipient_id)
        }
    }

    pub fn service_provider(
        recipient_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider =
            ServiceProvider::new(connection_manager.clone(), get_test_settings(""));
        service_provider.recipient_service = Box::new(recipient_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_delete_recipient_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_delete_recipient_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: String!) {
            deleteRecipient(recipientId: $input) {
            ... on DeleteResponse {
                id
                }
            }
          }
        "#;

        let variables = Some(json!({
          "input": "invalid_recipient_id"
        }));
        // Record Not Found
        let test_service = TestService(Box::new(|_| {
            Err(DeleteRecipientError::RecipientDoesNotExist)
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
    async fn test_graphql_delete_recipient_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_delete_recipient_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: String!) {
            deleteRecipient(recipientId: $input) {
              ... on DeleteResponse {
                id
              }
            }
          }
        "#;

        let variables = Some(json!({
          "input": "recipient_id"
        }));

        let test_service = TestService(Box::new(|_| Ok("recipient_id".to_owned())));

        let expected = json!({
            "deleteRecipient": {
                "id": "recipient_id",
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
