use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    notification_query::create::CreateNotificationQuery,
};

use crate::types::NotificationQueryNode;

use super::{map_error, ModifyNotificationQueryResponse};

#[derive(InputObject, Clone)]
pub struct CreateNotificationQueryInput {
    pub id: String,
    pub name: String,
    pub reference_name: String,
}

pub fn create_notification_query(
    ctx: &Context<'_>,
    input: CreateNotificationQueryInput,
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
        .create_notification_query(&service_context, input.into())
    {
        Ok(notification_query) => Ok(ModifyNotificationQueryResponse::Response(
            NotificationQueryNode::from_domain(notification_query),
        )),
        Err(error) => map_error(error),
    }
}

impl From<CreateNotificationQueryInput> for CreateNotificationQuery {
    fn from(
        CreateNotificationQueryInput {
            id,
            name,
            reference_name,
        }: CreateNotificationQueryInput,
    ) -> Self {
        CreateNotificationQuery {
            id,
            name,
            reference_name,
        }
    }
}
