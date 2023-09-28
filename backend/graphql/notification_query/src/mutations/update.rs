use async_graphql::*;

use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_query::update::UpdateNotificationQuery,
};

use crate::types::NotificationQueryNode;

use super::{map_error, ModifyNotificationQueryResponse};
#[derive(InputObject, Clone)]
pub struct UpdateNotificationQueryInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub query: Option<String>,
    pub required_parameters: Option<Vec<String>>,
}

pub fn update_notification_query(
    ctx: &Context<'_>,
    input: UpdateNotificationQueryInput,
) -> Result<ModifyNotificationQueryResponse> {
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
        .update_notification_query(&service_context, input.into())
    {
        Ok(config_row) => Ok(ModifyNotificationQueryResponse::Response(
            NotificationQueryNode::from_domain(config_row),
        )),
        Err(error) => map_error(error),
    }
}

impl From<UpdateNotificationQueryInput> for UpdateNotificationQuery {
    fn from(
        UpdateNotificationQueryInput {
            id,
            name,
            description,
            query,
            required_parameters,
        }: UpdateNotificationQueryInput,
    ) -> Self {
        UpdateNotificationQuery {
            id,
            name,
            description,
            query,
            required_parameters,
        }
    }
}
