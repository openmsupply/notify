use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    sql_recipient_list::delete::DeleteSqlRecipientListError as ServiceError,
};

pub fn delete_sql_recipient_list(
    ctx: &Context<'_>,
    sql_recipient_list_id: &str,
) -> Result<DeleteSqlRecipientListResponse> {
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
        .delete_sql_recipient_list(&service_context, sql_recipient_list_id)
    {
        Ok(sql_recipient_list_id) => Ok(DeleteSqlRecipientListResponse::Response(DeleteResponse(
            sql_recipient_list_id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteSqlRecipientListResponse {
    Response(DeleteResponse),
}

fn map_error(error: ServiceError) -> Result<DeleteSqlRecipientListResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Structured Errors
        // Standard Graphql Errors
        ServiceError::SqlRecipientListDoesNotExist => BadUserInput(formatted_error),
        ServiceError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}
