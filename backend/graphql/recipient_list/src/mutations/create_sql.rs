use crate::types::SqlRecipientListNode;

use super::{map_sql_error, ModifySqlRecipientListResponse};
use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};

use service::{
    auth::{Resource, ResourceAccessRequest},
    sql_recipient_list::create::CreateSqlRecipientList,
};

#[derive(InputObject, Clone)]
pub struct CreateSqlRecipientListInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub parameters: Vec<String>, // This will be saved as a JSON array object containing parameter names ["param1", "param2"] all params are assumed to be strings
}

pub fn create_sql_recipient_list(
    ctx: &Context<'_>,
    input: CreateSqlRecipientListInput,
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
        .create_sql_recipient_list(&service_context, input.into())
    {
        Ok(recipient_list) => Ok(ModifySqlRecipientListResponse::Response(
            SqlRecipientListNode::from_domain(recipient_list),
        )),
        Err(error) => map_sql_error(error),
    }
}

impl From<CreateSqlRecipientListInput> for CreateSqlRecipientList {
    fn from(
        CreateSqlRecipientListInput {
            id,
            name,
            description,
            query,
            parameters,
        }: CreateSqlRecipientListInput,
    ) -> Self {
        CreateSqlRecipientList {
            id,
            name,
            description,
            query,
            required_parameters: parameters,
        }
    }
}
