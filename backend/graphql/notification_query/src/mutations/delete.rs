use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_query::delete::DeleteNotificationQueryError,
};

pub fn delete_notification_query(
    ctx: &Context<'_>,
    id: &str,
) -> Result<DeleteNotificationQueryResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    match service_context
        .service_provider
        .notification_query_service
        .delete_notification_query(&service_context, id)
    {
        Ok(id) => Ok(DeleteNotificationQueryResponse::Response(DeleteResponse(
            id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteNotificationQueryResponse {
    Response(DeleteResponse),
}

fn map_error(error: DeleteNotificationQueryError) -> Result<DeleteNotificationQueryResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        DeleteNotificationQueryError::NotificationQueryDoesNotExist => {
            BadUserInput(formatted_error)
        }
        DeleteNotificationQueryError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}
