mod mutations;
use self::mutations::*;
mod types;
use self::types::*;

use async_graphql::*;
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::*;
use repository::NotificationConfigFilter;
use repository::PaginationOption;
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct NotificationConfigQueries;

#[Object]
impl NotificationConfigQueries {
    pub async fn notification_configs(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<NotificationConfigFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<NotificationConfigSortInput>>,
    ) -> Result<NotificationConfigsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_context = ctx.service_context(Some(&user))?;

        let configs = service_context
            .service_provider
            .notification_config_service
            .get_notification_configs(
                &service_context,
                page.map(PaginationOption::from),
                filter.map(NotificationConfigFilter::from),
                // Currently only one sort option is supported, use the first from the list.
                sort.and_then(|mut sort_list| sort_list.pop())
                    .map(|sort| sort.to_domain()),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(NotificationConfigsResponse::Response(
            NotificationConfigConnector::from_domain(configs),
        ))
    }
}

#[derive(Default, Clone)]
pub struct NotificationConfigMutations;

#[Object]
impl NotificationConfigMutations {
    async fn create_notification_config(
        &self,
        ctx: &Context<'_>,
        input: CreateNotificationConfigInput,
    ) -> Result<ModifyNotificationConfigResponse> {
        create_notification_config(ctx, input)
    }

    async fn update_notification_config(
        &self,
        ctx: &Context<'_>,
        input: UpdateNotificationConfigInput,
    ) -> Result<ModifyNotificationConfigResponse> {
        update_notification_config(ctx, input)
    }

    async fn delete_notification_config(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<DeleteNotificationConfigResponse> {
        delete_notification_config(ctx, &id)
    }
}
