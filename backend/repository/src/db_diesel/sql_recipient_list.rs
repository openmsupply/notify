use super::{
    sql_recipient_list_row::{
        sql_recipient_list, sql_recipient_list::dsl as sql_recipient_list_dsl,
    },
    DBType, SqlRecipientListRow, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case, apply_string_filter},
    repository_error::RepositoryError,
    EqualFilter, Pagination, Sort, StringFilter,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type SqlRecipientList = SqlRecipientListRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SqlRecipientListFilter {
    pub id: Option<EqualFilter<String>>,
    pub name: Option<StringFilter>,
    pub search: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum SqlRecipientListSortField {
    Name,
    Id,
}

pub type SqlRecipientListSort = Sort<SqlRecipientListSortField>;

pub struct SqlRecipientListRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> SqlRecipientListRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        SqlRecipientListRepository { connection }
    }

    pub fn count(&self, filter: Option<SqlRecipientListFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: SqlRecipientListFilter,
    ) -> Result<Vec<SqlRecipientList>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(
        &self,
        filter: SqlRecipientListFilter,
    ) -> Result<Option<SqlRecipientList>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<SqlRecipientListFilter>,
        sort: Option<SqlRecipientListSort>,
    ) -> Result<Vec<SqlRecipientList>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                SqlRecipientListSortField::Name => {
                    apply_sort_no_case!(query, sort, sql_recipient_list_dsl::name);
                }
                SqlRecipientListSortField::Id => {
                    apply_sort_no_case!(query, sort, sql_recipient_list_dsl::id);
                }
            }
        } else {
            query = query.order(sql_recipient_list_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<SqlRecipientList>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedRecipientQuery = IntoBoxed<'static, sql_recipient_list::table, DBType>;

fn create_filtered_query(filter: Option<SqlRecipientListFilter>) -> BoxedRecipientQuery {
    let mut query = sql_recipient_list_dsl::sql_recipient_list.into_boxed();

    if let Some(f) = filter {
        let SqlRecipientListFilter { id, name, search } = f;

        apply_equal_filter!(query, id, sql_recipient_list_dsl::id);
        apply_string_filter!(query, name, sql_recipient_list_dsl::name);

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(
                sql_recipient_list_dsl::name
                    .like(search_term.clone())
                    .or(sql_recipient_list_dsl::description.like(search_term)),
            );
        }
    }

    query
}

impl SqlRecipientListFilter {
    pub fn new() -> SqlRecipientListFilter {
        SqlRecipientListFilter::default()
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
