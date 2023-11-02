use async_graphql::{Enum, InputObject};
use graphql_core::generic_filters::{EqualFilterStringInput, StringFilterInput};
use repository::{
    EqualFilter, NotificationEventFilter, NotificationEventSort, NotificationEventSortField,
};

use super::EventStatus;

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum NotificationEventSortFieldInput {
    Title,
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
    pub title: Option<StringFilterInput>,
    pub search: Option<String>,
    pub status: Option<EqualFilterEventStatusInput>,
}

impl From<NotificationEventFilterInput> for NotificationEventFilter {
    fn from(f: NotificationEventFilterInput) -> Self {
        NotificationEventFilter {
            id: f.id.map(EqualFilter::from),
            // title: f.title.map(StringFilter::from),
            search: f.search,
            // status: f
            //     .status
            //     .map(|t| map_filter!(t, NotificationEventStatus::to_domain)),
        }
    }
}
