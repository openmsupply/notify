use async_graphql::{Enum, InputObject};
use graphql_core::generic_filters::{EqualFilterStringInput, StringFilterInput};
use repository::{
    EqualFilter, NotificationQueryFilter, NotificationQuerySort, NotificationQuerySortField,
    StringFilter,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum NotificationQuerySortFieldInput {
    Name,
}

#[derive(InputObject)]
pub struct NotificationQuerySortInput {
    /// Sort query result by `key`
    key: NotificationQuerySortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}
impl NotificationQuerySortInput {
    pub fn to_domain(self) -> NotificationQuerySort {
        use NotificationQuerySortField as to;
        use NotificationQuerySortFieldInput as from;
        let key = match self.key {
            from::Name => to::Name,
        };

        NotificationQuerySort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Clone, InputObject)]
pub struct NotificationQueryFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub name: Option<StringFilterInput>,
    pub search: Option<String>,
}

impl From<NotificationQueryFilterInput> for NotificationQueryFilter {
    fn from(f: NotificationQueryFilterInput) -> Self {
        NotificationQueryFilter {
            id: f.id.map(EqualFilter::from),
            name: f.name.map(StringFilter::from),
            search: f.search,
        }
    }
}
