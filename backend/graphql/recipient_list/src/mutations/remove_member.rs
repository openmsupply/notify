use async_graphql::*;
use graphql_types::types::IdResponse;

use super::{map_list_member_error, ModifyRecipientListMembersResponse};
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::remove_member::RemoveRecipientFromList,
};

pub fn remove_recipient_from_list(
    ctx: &Context<'_>,
    input: RemoveRecipientFromListInput,
) -> Result<ModifyRecipientListMembersResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    let service = &service_context.service_provider.recipient_list_service;

    match service.remove_recipient_from_list(&service_context, input.into()) {
        Ok(member) => Ok(ModifyRecipientListMembersResponse::Response(IdResponse(
            member.recipient_list_id,
        ))),
        Err(error) => map_list_member_error(error),
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
