use async_graphql::*;

use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::RecipientListNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::update::UpdateRecipientList,
};

use super::{map_error, ModifyRecipientListResponse};

pub fn update_recipient_list(
    ctx: &Context<'_>,
    input: UpdateRecipientListInput,
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
        .update_recipient_list(&service_context, input.into())
    {
        Ok(recipient_list_row) => Ok(ModifyRecipientListResponse::Response(
            RecipientListNode::from_domain(recipient_list_row),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct UpdateRecipientListInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sql_query: Option<String>,
}

impl From<UpdateRecipientListInput> for UpdateRecipientList {
    fn from(
        UpdateRecipientListInput {
            id,
            name,
            description,
            sql_query,
        }: UpdateRecipientListInput,
    ) -> Self {
        UpdateRecipientList {
            id,
            name,
            description,
            sql_query,
        }
    }
}
