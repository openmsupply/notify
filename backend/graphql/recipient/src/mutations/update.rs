use async_graphql::*;

use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::RecipientNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient::update::UpdateRecipient,
    recipient::ModifyRecipientError,
};

pub fn update_recipient(
    ctx: &Context<'_>,
    input: UpdateRecipientInput,
) -> Result<UpdateRecipientResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;
    match ctx
        .service_provider()
        .recipient_service
        .update_recipient(&service_context, input.into())
    {
        Ok(recipient_row) => Ok(UpdateRecipientResponse::Response(
            RecipientNode::from_domain(recipient_row),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct UpdateRecipientInput {
    pub id: String,
    pub name: Option<String>,
    pub to_address: Option<String>,
}

impl From<UpdateRecipientInput> for UpdateRecipient {
    fn from(
        UpdateRecipientInput {
            id,
            name,
            to_address,
        }: UpdateRecipientInput,
    ) -> Self {
        UpdateRecipient {
            id,
            name,
            to_address,
        }
    }
}

#[derive(Union)]
pub enum UpdateRecipientResponse {
    Response(RecipientNode),
}

fn map_error(error: ModifyRecipientError) -> Result<UpdateRecipientResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyRecipientError::RecipientDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientError::RecipientAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyRecipientError::DatabaseError(_) => InternalError(formatted_error),
        ModifyRecipientError::GenericError(s) => InternalError(s),
    };

    Err(graphql_error.extend())
}

#[cfg(test)]
mod test {

    use async_graphql::EmptyMutation;
    use graphql_core::{
        assert_graphql_query, assert_standard_graphql_error, test_helpers::setup_graphql_test,
    };
    use repository::{
        mock::MockDataInserts, NotificationType, Recipient, RecipientRow, StorageConnectionManager,
    };
    use serde_json::json;

    use service::{
        recipient::{update::UpdateRecipient, ModifyRecipientError, RecipientServiceTrait},
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientMutations;

    type UpdateRecipientMethod =
        dyn Fn(UpdateRecipient) -> Result<Recipient, ModifyRecipientError> + Sync + Send;

    pub struct TestService(pub Box<UpdateRecipientMethod>);

    impl RecipientServiceTrait for TestService {
        fn update_recipient(
            &self,
            _: &ServiceContext,
            input: UpdateRecipient,
        ) -> Result<Recipient, ModifyRecipientError> {
            (self.0)(input)
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
    async fn test_graphql_update_recipient_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_update_recipient_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: UpdateRecipientInput!) {
            updateRecipient(input: $input) {
                ... on RecipientNode {
                    id
                    name
                    toAddress
                  }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "robs_id",
            "name": "new rob",
            "toAddress": "newrob@email.com",
          }
        }));

        // Record Doesn't Exist
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientError::RecipientDoesNotExist)
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

        // Updated record does not exist (this shouldn't happen, but want to test internal error)
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientError::ModifiedRecordNotFound)
        }));
        let expected_message = "Internal error";
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
    async fn test_graphql_update_recipient_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_update_recipient_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: UpdateRecipientInput!) {
            updateRecipient(input: $input) {
              ... on RecipientNode {
                id
                name
              }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "robs_id",
            "name": "new name for rob"
          }
        }));

        let test_service = TestService(Box::new(|_| {
            Ok(RecipientRow {
                id: "robs_id".to_string(),
                name: "new name for rob".to_string(),
                to_address: "rob@email.com".to_string(),
                notification_type: NotificationType::Email,
            })
        }));

        let expected = json!({
            "updateRecipient": {
                "id": "robs_id",
                "name": "new name for rob",
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
