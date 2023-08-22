use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use graphql_types::types::DeleteResponse;
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_config::delete::DeleteNotificationConfigError,
};

pub fn delete_notification_config(
    ctx: &Context<'_>,
    id: &str,
) -> Result<DeleteNotificationConfigResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    let service_context = ctx.service_context(Some(&user))?;

    match service_context
        .service_provider
        .notification_config_service
        .delete_notification_config(&service_context, id)
    {
        Ok(id) => Ok(DeleteNotificationConfigResponse::Response(DeleteResponse(
            id,
        ))),
        Err(error) => map_error(error),
    }
}

#[derive(Union)]
pub enum DeleteNotificationConfigResponse {
    Response(DeleteResponse),
}

fn map_error(error: DeleteNotificationConfigError) -> Result<DeleteNotificationConfigResponse> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        DeleteNotificationConfigError::NotificationConfigDoesNotExist => {
            BadUserInput(formatted_error)
        }
        DeleteNotificationConfigError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}
