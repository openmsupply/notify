mod types;
use self::types::*;

use async_graphql::*;
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};

use repository::NotificationEventFilter;
use repository::PaginationOption;
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct NotificationEventQueries;

#[Object]
impl NotificationEventQueries {
    pub async fn notification_events(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<NotificationEventFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<NotificationEventSortInput>>,
    ) -> Result<NotificationEventsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_context = ctx.service_context(Some(&user))?;

        let configs = service_context
            .service_provider
            .notification_event_service
            .get_notification_events(
                &service_context,
                page.map(PaginationOption::from),
                filter.map(NotificationEventFilter::from),
                // Currently only one sort option is supported, use the first from the list.
                sort.and_then(|mut sort_list| sort_list.pop())
                    .map(|sort| sort.to_domain()),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(NotificationEventsResponse::Response(
            NotificationEventConnector::from_domain(configs),
        ))
    }
}
