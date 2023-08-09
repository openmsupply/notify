use async_graphql::*;

use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::RecipientListNode;
use repository::RecipientList;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::add_member::AddRecipientToList,
    recipient_list::ModifyRecipientListError,
};

pub fn add_recipient_to_list(
    ctx: &Context<'_>,
    input: AddRecipientToListInput,
) -> Result<AddRecipientToListResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;
    match service_context
        .service_provider
        .recipient_list_service
        .add_recipient_to_list(&service_context, input.into())
    {
        Ok(recipient_list_member) => Ok(AddRecipientToListResponse::Response(
            // TODO: what do we wanna do here hm
            RecipientListNode::from_domain(RecipientList {
                id: recipient_list_member.recipient_list_id.clone(),
                ..Default::default()
            }),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct AddRecipientToListInput {
    pub id: String,
    pub recipient_id: String,
    pub recipient_list_id: String,
}

impl From<AddRecipientToListInput> for AddRecipientToList {
    fn from(
        AddRecipientToListInput {
            id,
            recipient_id,
            recipient_list_id,
        }: AddRecipientToListInput,
    ) -> Self {
        AddRecipientToList {
            id,
            recipient_id,
            recipient_list_id,
        }
    }
}

#[derive(Union)]
pub enum AddRecipientToListResponse {
    Response(RecipientListNode),
}

fn map_error(error: ModifyRecipientListError) -> Result<AddRecipientToListResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyRecipientListError::RecipientListDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListMemberAlreadyExists => BadUserInput(formatted_error),
        ModifyRecipientListError::RecipientListMemberDoesNotExist => BadUserInput(formatted_error),
        ModifyRecipientListError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyRecipientListError::DatabaseError(_) => InternalError(formatted_error),
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
            update::UpdateRecipientList, ModifyRecipientListError, RecipientListServiceTrait,
        },
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    use crate::RecipientListMutations;

    type UpdateRecipientListMethod = dyn Fn(UpdateRecipientList) -> Result<RecipientList, ModifyRecipientListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<UpdateRecipientListMethod>);

    impl RecipientListServiceTrait for TestService {
        fn update_recipient_list(
            &self,
            _: &ServiceContext,
            input: UpdateRecipientList,
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
    async fn test_graphql_update_recipient_list_errors() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_update_recipient_list_errors",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: UpdateRecipientListInput!) {
            updateRecipientList(input: $input) {
                ... on RecipientListNode {
                    id
                    name
                  }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "group_id",
            "name": "new group name",
          }
        }));

        // Record Doesn't Exist
        let test_service = TestService(Box::new(|_| {
            Err(ModifyRecipientListError::RecipientListDoesNotExist)
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
    async fn test_graphql_update_recipient_list_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            EmptyMutation,
            RecipientListMutations,
            "test_graphql_update_recipient_list_success",
            MockDataInserts::none(),
        )
        .await;

        let mutation = r#"
        mutation ($input: UpdateRecipientListInput!) {
            updateRecipientList(input: $input) {
              ... on RecipientListNode {
                id
                name
              }
            }
          }
        "#;

        let variables = Some(json!({
          "input": {
            "id": "group_id",
            "name": "new name for group"
          }
        }));

        let test_service = TestService(Box::new(|_| {
            Ok(RecipientListRow {
                id: "group_id".to_string(),
                name: "new name for group".to_string(),
                description: "description".to_string(),
            })
        }));

        let expected = json!({
            "updateRecipientList": {
                "id": "group_id",
                "name": "new name for group",
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
