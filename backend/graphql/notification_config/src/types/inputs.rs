use async_graphql::{Enum, InputObject};
use graphql_core::{
    generic_filters::{EqualFilterStringInput, StringFilterInput},
    map_filter,
};
use graphql_types::types::{ConfigKind, ConfigStatus};
use repository::{
    EqualFilter, NotificationConfigFilter, NotificationConfigSort, NotificationConfigSortField,
    StringFilter,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum NotificationConfigSortFieldInput {
    Title,
    Kind,
    Status,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterConfigKindInput {
    pub equal_to: Option<ConfigKind>,
    pub equal_any: Option<Vec<ConfigKind>>,
    pub not_equal_to: Option<ConfigKind>,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterConfigStatusInput {
    pub equal_to: Option<ConfigStatus>,
    pub equal_any: Option<Vec<ConfigStatus>>,
    pub not_equal_to: Option<ConfigStatus>,
}

#[derive(InputObject)]
pub struct NotificationConfigSortInput {
    /// Sort query result by `key`
    key: NotificationConfigSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}
impl NotificationConfigSortInput {
    pub fn to_domain(self) -> NotificationConfigSort {
        use NotificationConfigSortField as to;
        use NotificationConfigSortFieldInput as from;
        let key = match self.key {
            from::Title => to::Title,
            from::Kind => to::Kind,
            from::Status => to::Status,
        };

        NotificationConfigSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Clone, InputObject)]
pub struct NotificationConfigFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub title: Option<StringFilterInput>,
    pub kind: Option<EqualFilterConfigKindInput>,
    pub search: Option<String>,
    pub status: Option<EqualFilterConfigStatusInput>,
}

impl From<NotificationConfigFilterInput> for NotificationConfigFilter {
    fn from(f: NotificationConfigFilterInput) -> Self {
        NotificationConfigFilter {
            id: f.id.map(EqualFilter::from),
            title: f.title.map(StringFilter::from),
            kind: f.kind.map(|t| map_filter!(t, ConfigKind::to_domain)),
            search: f.search,
            status: f.status.map(|t| map_filter!(t, ConfigStatus::to_domain)),
        }
    }
}
