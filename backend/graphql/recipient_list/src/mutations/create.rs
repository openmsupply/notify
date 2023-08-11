use super::{map_recipient_list_error, ModifyRecipientListResponse};
use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::RecipientListNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::create::CreateRecipientList,
};

pub fn create_recipient_list(
    ctx: &Context<'_>,
    input: CreateRecipientListInput,
) -> Result<ModifyRecipientListResponse> {
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
        .create_recipient_list(&service_context, input.into())
    {
        Ok(recipient_list) => Ok(ModifyRecipientListResponse::Response(
            RecipientListNode::from_domain(recipient_list),
        )),
        Err(error) => map_recipient_list_error(error),
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
