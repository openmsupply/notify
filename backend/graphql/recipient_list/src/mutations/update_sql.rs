use async_graphql::*;

use crate::SqlRecipientListNode;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    sql_recipient_list::update::UpdateSqlRecipientList,
};

use super::{map_sql_error, ModifySqlRecipientListResponse};

pub fn update_sql_recipient_list(
    ctx: &Context<'_>,
    input: UpdateSqlRecipientListInput,
) -> Result<ModifySqlRecipientListResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;
    match service_context
        .service_provider
        .sql_recipient_list_service
        .update_sql_recipient_list(&service_context, input.into())
    {
        Ok(sql_recipient_list_row) => Ok(ModifySqlRecipientListResponse::Response(
            SqlRecipientListNode::from_domain(sql_recipient_list_row),
        )),
        Err(error) => map_sql_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct UpdateSqlRecipientListInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<UpdateSqlRecipientListInput> for UpdateSqlRecipientList {
    fn from(
        UpdateSqlRecipientListInput {
            id,
            name,
            description,
        }: UpdateSqlRecipientListInput,
    ) -> Self {
        UpdateSqlRecipientList {
            id,
            name,
            description,
        }
    }
}
