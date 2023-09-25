use async_graphql::*;

use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use graphql_types::types::{ConfigStatus, NotificationConfigNode};
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_config::update::UpdateNotificationConfig,
};

use super::{map_error, ModifyNotificationConfigResponse};
#[derive(InputObject, Clone)]
pub struct UpdateNotificationConfigInput {
    pub id: String,
    pub title: Option<String>,
    pub configuration_data: Option<String>,
    pub parameters: Option<String>,
    pub status: Option<ConfigStatus>,
    pub recipient_ids: Option<Vec<String>>,
    pub recipient_list_ids: Option<Vec<String>>,
    pub sql_recipient_list_ids: Option<Vec<String>>,
}

pub fn update_notification_config(
    ctx: &Context<'_>,
    input: UpdateNotificationConfigInput,
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
        .update_notification_config(&service_context, input.into())
    {
        Ok(config_row) => Ok(ModifyNotificationConfigResponse::Response(
            NotificationConfigNode::from_domain(config_row),
        )),
        Err(error) => map_error(error),
    }
}

impl From<UpdateNotificationConfigInput> for UpdateNotificationConfig {
    fn from(
        UpdateNotificationConfigInput {
            id,
            title,
            configuration_data,
            status,
            parameters,
            recipient_ids,
            recipient_list_ids,
            sql_recipient_list_ids,
        }: UpdateNotificationConfigInput,
    ) -> Self {
        UpdateNotificationConfig {
            id,
            title,
            configuration_data,
            status: status.map(ConfigStatus::to_domain),
            parameters,
            recipient_ids,
            recipient_list_ids,
            sql_recipient_list_ids,
        }
    }
}
