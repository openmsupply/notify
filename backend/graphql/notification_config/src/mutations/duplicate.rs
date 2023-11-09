use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::{ConfigKind, ConfigStatus, NotificationConfigNode};
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_config::duplicate::DuplicateNotificationConfig,
};

use super::{map_error, ModifyNotificationConfigResponse};

#[derive(InputObject, Clone)]
pub struct DuplicateNotificationConfigInput {
    pub id: String,
    pub title: String,
    pub kind: ConfigKind,
    pub status: ConfigStatus,
    pub configuration_data: Option<String>,
    pub parameters: Option<String>,
    pub recipient_ids: Option<Vec<String>>,
    pub recipient_list_ids: Option<Vec<String>>,
    pub sql_recipient_list_ids: Option<Vec<String>>,
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
        DuplicateNotificationConfigInput {
            id,
            title,
            kind,
            status,
            configuration_data,
            parameters,
            recipient_ids,
            recipient_list_ids,
            sql_recipient_list_ids,
        }: DuplicateNotificationConfigInput,
    ) -> Self {
        DuplicateNotificationConfig {
            id,
            title,
            kind: ConfigKind::to_domain(kind),
            status: ConfigStatus::to_domain(status),
            configuration_data,
            parameters,
            recipient_ids,
            recipient_list_ids,
            sql_recipient_list_ids,
        }
    }
}
