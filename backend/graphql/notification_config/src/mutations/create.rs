use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::{ConfigKind, NotificationConfigNode};
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_config::create::CreateNotificationConfig,
};

use super::{map_error, ModifyNotificationConfigResponse};

pub fn create_notification_config(
    ctx: &Context<'_>,
    input: CreateNotificationConfigInput,
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
        .create_notification_config(&service_context, input.into())
    {
        Ok(notification_config) => Ok(ModifyNotificationConfigResponse::Response(
            NotificationConfigNode::from_domain(notification_config),
        )),
        Err(error) => map_error(error),
    }
}

#[derive(InputObject, Clone)]
pub struct CreateNotificationConfigInput {
    pub id: String,
    pub title: String,
    pub kind: ConfigKind,
    pub recipient_ids: Vec<String>,
    pub recipient_list_ids: Vec<String>,
    pub configuration_data: String,
}

impl From<CreateNotificationConfigInput> for CreateNotificationConfig {
    fn from(
        CreateNotificationConfigInput {
            id,
            title,
            kind,
            configuration_data,
            recipient_ids,
            recipient_list_ids,
        }: CreateNotificationConfigInput,
    ) -> Self {
        CreateNotificationConfig {
            id,
            title,
            kind: ConfigKind::to_domain(kind),
            configuration_data,
            recipient_ids,
            recipient_list_ids,
        }
    }
}
