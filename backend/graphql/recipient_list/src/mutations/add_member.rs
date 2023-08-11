use async_graphql::*;
use graphql_types::types::IdResponse;

use super::{map_list_member_error, ModifyRecipientListMembersResponse};
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::add_member::AddRecipientToList,
};

pub fn add_recipient_to_list(
    ctx: &Context<'_>,
    input: AddRecipientToListInput,
) -> Result<ModifyRecipientListMembersResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    let service = &service_context.service_provider.recipient_list_service;

    match service.add_recipient_to_list(&service_context, input.into()) {
        Ok(member) => Ok(ModifyRecipientListMembersResponse::Response(IdResponse(
            member.recipient_id,
        ))),
        Err(error) => map_list_member_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct AddRecipientToListInput {
    pub recipient_id: String,
    pub recipient_list_id: String,
}

impl From<AddRecipientToListInput> for AddRecipientToList {
    fn from(
        AddRecipientToListInput {
            recipient_id,
            recipient_list_id,
        }: AddRecipientToListInput,
    ) -> Self {
        AddRecipientToList {
            recipient_id,
            recipient_list_id,
        }
    }
}
