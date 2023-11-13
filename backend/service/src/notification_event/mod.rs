pub mod query;
mod tests;

use self::query::{get_notification_event, get_notification_events};

use super::{ListError, ListResult};
use crate::{service_provider::ServiceContext, SingleRecordError};

use repository::{
    NotificationEvent, NotificationEventFilter, NotificationEventSort, PaginationOption,
};

pub trait NotificationEventServiceTrait: Sync + Send {
    fn get_notification_events(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<NotificationEventFilter>,
        sort: Option<NotificationEventSort>,
    ) -> Result<ListResult<NotificationEvent>, ListError> {
        get_notification_events(ctx, pagination, filter, sort)
    }

    fn get_notification_event(
        &self,
        ctx: &ServiceContext,
        notification_event_id: String,
    ) -> Result<NotificationEvent, SingleRecordError> {
        get_notification_event(ctx, notification_event_id)
    }
}

pub struct NotificationEventService {}
impl NotificationEventServiceTrait for NotificationEventService {}
