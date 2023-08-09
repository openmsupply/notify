use async_graphql::*;

use super::{map_error, ModifyRecipientListResponse};
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::RecipientListNode;
use repository::RecipientList;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::add_member::AddRecipientToList,
};

pub fn add_recipient_to_list(
    ctx: &Context<'_>,
    input: AddRecipientToListInput,
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
        .add_recipient_to_list(&service_context, input.into())
    {
        Ok(recipient_list_member) => Ok(ModifyRecipientListResponse::Response(
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
