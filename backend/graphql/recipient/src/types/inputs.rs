use async_graphql::{Enum, InputObject};
use graphql_core::{generic_filters::EqualFilterStringInput, map_filter};
use graphql_types::types::NotificationTypeNode;
use repository::{EqualFilter, RecipientFilter, RecipientSort, RecipientSortField};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum RecipientSortFieldInput {
    Name,
    ToAddress,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterNotificationTypeInput {
    pub equal_to: Option<NotificationTypeNode>,
    pub equal_any: Option<Vec<NotificationTypeNode>>,
    pub not_equal_to: Option<NotificationTypeNode>,
}

#[derive(InputObject)]
pub struct RecipientSortInput {
    /// Sort query result by `key`
    key: RecipientSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}
impl RecipientSortInput {
    pub fn to_domain(self) -> RecipientSort {
        use RecipientSortField as to;
        use RecipientSortFieldInput as from;
        let key = match self.key {
            from::Name => to::Name,
            from::ToAddress => to::ToAddress,
        };

        RecipientSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Clone, InputObject)]
pub struct RecipientFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub search: Option<String>,
    pub notification_type: Option<EqualFilterNotificationTypeInput>,
}

impl From<RecipientFilterInput> for RecipientFilter {
    fn from(f: RecipientFilterInput) -> Self {
        RecipientFilter {
            id: f.id.map(EqualFilter::from),
            notification_type: f
                .notification_type
                .map(|t| map_filter!(t, NotificationTypeNode::to_domain)),
            search: f.search,
        }
    }
}
