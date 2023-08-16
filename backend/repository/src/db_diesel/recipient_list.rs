use super::{
    recipient_list_row::{recipient_list, recipient_list::dsl as recipient_list_dsl},
    DBType, RecipientListRow, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case, apply_string_filter},
    repository_error::RepositoryError,
    EqualFilter, Pagination, Sort, StringFilter,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type RecipientList = RecipientListRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RecipientListFilter {
    pub id: Option<EqualFilter<String>>,
    pub name: Option<StringFilter>,
    pub search: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum RecipientListSortField {
    Name,
}

pub type RecipientListSort = Sort<RecipientListSortField>;

pub struct RecipientListRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientListRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientListRepository { connection }
    }

    pub fn count(&self, filter: Option<RecipientListFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: RecipientListFilter,
    ) -> Result<Vec<RecipientList>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(
        &self,
        filter: RecipientListFilter,
    ) -> Result<Option<RecipientList>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<RecipientListFilter>,
        sort: Option<RecipientListSort>,
    ) -> Result<Vec<RecipientList>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                RecipientListSortField::Name => {
                    apply_sort_no_case!(query, sort, recipient_list_dsl::name);
                }
            }
        } else {
            query = query.order(recipient_list_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<RecipientList>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedRecipientQuery = IntoBoxed<'static, recipient_list::table, DBType>;

fn create_filtered_query(filter: Option<RecipientListFilter>) -> BoxedRecipientQuery {
    let mut query = recipient_list_dsl::recipient_list.into_boxed();

    if let Some(f) = filter {
        let RecipientListFilter { id, name, search } = f;

        apply_equal_filter!(query, id, recipient_list_dsl::id);
        apply_string_filter!(query, name, recipient_list_dsl::name);

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(
                recipient_list_dsl::name
                    .like(search_term.clone())
                    .or(recipient_list_dsl::description.like(search_term)),
            );
        }
    }

    query
}

impl RecipientListFilter {
    pub fn new() -> RecipientListFilter {
        RecipientListFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }
    pub fn name(mut self, filter: StringFilter) -> Self {
        self.name = Some(filter);
        self
    }

    pub fn search(mut self, filter: String) -> Self {
        self.search = Some(filter);
        self
    }
}
