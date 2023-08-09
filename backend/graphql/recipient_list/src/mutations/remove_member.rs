use async_graphql::*;

use super::{map_error, ModifyRecipientListResponse};
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::RecipientListNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::{remove_member::RemoveRecipientFromList, ModifyRecipientListError},
};

pub fn remove_recipient_from_list(
    ctx: &Context<'_>,
    input: RemoveRecipientFromListInput,
) -> Result<ModifyRecipientListResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    let service = &service_context.service_provider.recipient_list_service;

    match service.remove_recipient_from_list(&service_context, input.into()) {
        Ok(member) => {
            // TODO: should this return recipient_list or just the member_id seeing as it deletes the member??
            // This is a recipient list mutation - if successful, we query for and return the recipient list node
            match service.get_recipient_list(&service_context, member.recipient_list_id.clone()) {
                Ok(recipient_list_row) => Ok(ModifyRecipientListResponse::Response(
                    RecipientListNode::from_domain(recipient_list_row),
                )),
                Err(error) => map_error(ModifyRecipientListError::from(error)),
            }
        }
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct RemoveRecipientFromListInput {
    pub recipient_id: String,
    pub recipient_list_id: String,
}

impl From<RemoveRecipientFromListInput> for RemoveRecipientFromList {
    fn from(
        RemoveRecipientFromListInput {
            recipient_id,
            recipient_list_id,
        }: RemoveRecipientFromListInput,
    ) -> Self {
        RemoveRecipientFromList {
            recipient_id,
            recipient_list_id,
        }
    }
}
