use async_graphql::{Enum, InputObject};
use graphql_core::generic_filters::{EqualFilterStringInput, StringFilterInput};
use repository::{
    EqualFilter, RecipientListFilter, RecipientListSort, RecipientListSortField, StringFilter,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum RecipientListSortFieldInput {
    Name,
}

#[derive(InputObject)]
pub struct RecipientListSortInput {
    /// Sort query result by `key`
    key: RecipientListSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}
impl RecipientListSortInput {
    pub fn to_domain(self) -> RecipientListSort {
        use RecipientListSortField as to;
        use RecipientListSortFieldInput as from;
        let key = match self.key {
            from::Name => to::Name,
        };

        RecipientListSort {
            key,
            desc: self.desc,
        }
    }
}

#[derive(Clone, InputObject)]
pub struct RecipientListFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub name: Option<StringFilterInput>,
    pub search: Option<String>,
}

impl From<RecipientListFilterInput> for RecipientListFilter {
    fn from(f: RecipientListFilterInput) -> Self {
        RecipientListFilter {
            id: f.id.map(EqualFilter::from),
            name: f.name.map(StringFilter::from),
            search: f.search,
        }
    }
}
