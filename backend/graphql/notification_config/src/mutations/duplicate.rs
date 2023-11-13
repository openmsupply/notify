use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::NotificationConfigNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_config::duplicate::DuplicateNotificationConfig,
};

use super::{map_error, ModifyNotificationConfigResponse};

#[derive(InputObject, Clone)]
pub struct DuplicateNotificationConfigInput {
    pub old_id: String,
    pub new_id: String,
}

pub fn duplicate_notification_config(
    ctx: &Context<'_>,
    input: DuplicateNotificationConfigInput,
) -> Result<ModifyNotificationConfigResponse> {
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
        .duplicate_notification_config(&service_context, input.into())
    {
        Ok(notification_config) => Ok(ModifyNotificationConfigResponse::Response(
            NotificationConfigNode::from_domain(notification_config),
        )),
        Err(error) => map_error(error),
    }
}

impl From<DuplicateNotificationConfigInput> for DuplicateNotificationConfig {
    fn from(
        DuplicateNotificationConfigInput { old_id, new_id }: DuplicateNotificationConfigInput,
    ) -> Self {
        DuplicateNotificationConfig { old_id, new_id }
    }
}
