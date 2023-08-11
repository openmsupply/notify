use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    recipient_list::delete::DeleteRecipientListError as ServiceError,
};

pub fn delete_recipient_list(
    ctx: &Context<'_>,
    recipient_list_id: &str,
) -> Result<DeleteRecipientListResponse> {
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
        .delete_recipient_list(&service_context, recipient_list_id)
    {
        Ok(recipient_list_id) => Ok(DeleteRecipientListResponse::Response(DeleteResponse(
            recipient_list_id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteRecipientListResponse {
    Response(DeleteResponse),
}

fn map_error(error: ServiceError) -> Result<DeleteRecipientListResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Structured Errors
        // Standard Graphql Errors
        ServiceError::RecipientListDoesNotExist => BadUserInput(formatted_error),
        ServiceError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}
