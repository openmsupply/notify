use async_graphql::{Enum, InputObject};
use graphql_core::{
    generic_filters::{DatetimeFilterInput, EqualFilterStringInput},
    map_filter,
};
use repository::{
    DatetimeFilter, EqualFilter, NotificationEventFilter, NotificationEventSort,
    NotificationEventSortField,
};

use super::EventStatus;

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum NotificationEventSortFieldInput {
    Title,
    CreatedAt,
    ToAddress,
    Message,
    NotificationType,
    Status,
    ErrorMessage,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterEventStatusInput {
    pub equal_to: Option<EventStatus>,
    pub equal_any: Option<Vec<EventStatus>>,
    pub not_equal_to: Option<EventStatus>,
}

#[derive(InputObject)]
pub struct NotificationEventSortInput {
    /// Sort query result by `key`
    key: NotificationEventSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}
impl NotificationEventSortInput {
    pub fn to_domain(self) -> NotificationEventSort {
        use NotificationEventSortField as to;
        use NotificationEventSortFieldInput as from;
        let key = match self.key {
            from::Title => to::Title,
            from::CreatedAt => to::CreatedAt,
            from::ToAddress => to::ToAddress,
            from::Message => to::Message,
            from::NotificationType => to::NotificationType,
            from::Status => to::Status,
            from::ErrorMessage => to::ErrorMessage,
        };

        NotificationEventSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Clone, InputObject)]
pub struct NotificationEventFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub notification_config_id: Option<EqualFilterStringInput>,
    pub search: Option<String>,
    pub status: Option<EqualFilterEventStatusInput>,
    pub created_at: Option<DatetimeFilterInput>,
}

impl From<NotificationEventFilterInput> for NotificationEventFilter {
    fn from(f: NotificationEventFilterInput) -> Self {
        NotificationEventFilter {
            id: f.id.map(EqualFilter::from),
            notification_config_id: f.notification_config_id.map(EqualFilter::from),
            search: f.search,
            status: f.status.map(|t| map_filter!(t, EventStatus::to_domain)),
            created_at: f.created_at.map(DatetimeFilter::from),
        }
    }
}
