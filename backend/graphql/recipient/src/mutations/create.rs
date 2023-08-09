use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::{NotificationTypeNode, RecipientNode};
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient::{create::CreateRecipient, ModifyRecipientError},
};

pub fn create_recipient(
    ctx: &Context<'_>,
    input: CreateRecipientInput,
) -> Result<CreateRecipientResponse> {
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
        .create_recipient(&service_context, input.into())
    {
        Ok(recipient) => Ok(CreateRecipientResponse::Response(
            RecipientNode::from_domain(recipient),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct CreateRecipientInput {
    pub id: String,
    pub name: String,
    pub to_address: String,
    pub notification_type: NotificationTypeNode,
}

impl From<CreateRecipientInput> for CreateRecipient {
    fn from(
        CreateRecipientInput {
            id,
            name,
            to_address,
            notification_type,
        }: CreateRecipientInput,
    ) -> Self {
        CreateRecipient {
            id,
            name,
            to_address,
            notification_type: NotificationTypeNode::to_domain(notification_type),
        }
    }
}

#[derive(Union)]
pub enum CreateRecipientResponse {
    Response(RecipientNode),
}

fn map_error(error: ModifyRecipientError) -> Result<CreateRecipientResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyRecipientError::RecipientAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientError::RecipientDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientError::DatabaseError(_) => InternalError(formatted_error),
        ModifyRecipientError::ModifiedRecordNotFound => InternalError(formatted_error),
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
        recipient::{create::CreateRecipient, ModifyRecipientError, RecipientServiceTrait},
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientMutations;

    type CreateRecipientMethod =
        dyn Fn(CreateRecipient) -> Result<Recipient, ModifyRecipientError> + Sync + Send;

    pub struct TestService(pub Box<CreateRecipientMethod>);

    impl RecipientServiceTrait for TestService {
        fn create_recipient(
            &self,
            _: &ServiceContext,
            input: CreateRecipient,
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
    async fn test_graphql_create_recipient_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_create_recipient_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: CreateRecipientInput!) {
            createRecipient(input: $input) {
                ... on RecipientNode {
                    id
                    name
                    toAddress
                    notificationType
                  }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "robs_id",
            "name": "rob",
            "toAddress": "rob@email.com",
            "notificationType": "EMAIL"
          }
        }));

        // Record Already Exists
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientError::RecipientAlreadyExists)
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

        // Created record does not exists (this shouldn't happen, but want to test internal error)
        let mutation = r#"
         mutation ($input: CreateRecipientInput!) {
             createRecipient(input: $input) {
                ... on RecipientNode {
                    id
                    name
                    toAddress
                    notificationType
                }
             }
           }
         "#;

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
    async fn test_graphql_create_recipient_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientMutations,
            "test_graphql_create_recipient_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: CreateRecipientInput!) {
            createRecipient(input: $input) {
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
            "name": "rob",
            "toAddress": "rob@email.com",
            "notificationType": "EMAIL"
          }
        }));

        let test_service = TestService(Box::new(|_| {
            Ok(RecipientRow {
                id: "robs_id".to_string(),
                name: "rob".to_string(),
                to_address: "rob@email.com".to_string(),
                notification_type: NotificationType::Email,
            })
        }));

        let expected = json!({
            "createRecipient": {
                "id": "robs_id",
                "name": "rob",
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
