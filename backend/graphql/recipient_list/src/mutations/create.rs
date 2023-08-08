use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::RecipientListNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::{create::CreateRecipientList, ModifyRecipientListError},
};

pub fn create_recipient_list(
    ctx: &Context<'_>,
    input: CreateRecipientListInput,
) -> Result<CreateRecipientListResponse> {
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
        .create_recipient_list(&service_context, input.into())
    {
        Ok(recipient_list) => Ok(CreateRecipientListResponse::Response(
            RecipientListNode::from_domain(recipient_list),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct CreateRecipientListInput {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl From<CreateRecipientListInput> for CreateRecipientList {
    fn from(
        CreateRecipientListInput {
            id,
            name,
            description,
        }: CreateRecipientListInput,
    ) -> Self {
        CreateRecipientList {
            id,
            name,
            description,
        }
    }
}

#[derive(Union)]
pub enum CreateRecipientListResponse {
    Response(RecipientListNode),
}

fn map_error(error: ModifyRecipientListError) -> Result<CreateRecipientListResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyRecipientListError::RecipientListAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::DatabaseError(_) => InternalError(formatted_error),
        ModifyRecipientListError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyRecipientListError::GenericError(s) => InternalError(s),
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
        mock::MockDataInserts, RecipientList, RecipientListRow, StorageConnectionManager,
    };
    use serde_json::json;

    use service::{
        recipient_list::{
            create::CreateRecipientList, ModifyRecipientListError, RecipientListServiceTrait,
        },
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientListMutations;

    type CreateRecipientListMethod = dyn Fn(CreateRecipientList) -> Result<RecipientList, ModifyRecipientListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<CreateRecipientListMethod>);

    impl RecipientListServiceTrait for TestService {
        fn create_recipient_list(
            &self,
            _: &ServiceContext,
            input: CreateRecipientList,
        ) -> Result<RecipientList, ModifyRecipientListError> {
            (self.0)(input)
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
    async fn test_graphql_create_recipient_list_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_create_recipient_list_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: CreateRecipientListInput!) {
            createRecipientList(input: $input) {
                ... on RecipientListNode {
                    id
                    name
                    description
                  }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "cool_group_id",
            "name": "cool group",
            "description": "One very cool group"
          }
        }));

        // Record Already Exists
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientListError::RecipientListAlreadyExists)
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
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientListError::ModifiedRecordNotFound)
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
    async fn test_graphql_create_recipient_list_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_create_recipient_list_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: CreateRecipientListInput!) {
            createRecipientList(input: $input) {
              ... on RecipientListNode {
                id
                name
              }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "some-id",
            "name": "coolbean",
            "description": "The cool beans live here"
          }
        }));

        let test_service = TestService(Box::new(|_| {
            Ok(RecipientListRow {
                id: "some-id".to_string(),
                name: "coolbean".to_string(),
                description: "The cool beans live here".to_string(),
            })
        }));

        let expected = json!({
            "createRecipientList": {
                "id": "some-id",
                "name": "coolbean",
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
