use super::{
    notification_query_row::{
        notification_query, notification_query::dsl as notification_query_dsl,
    },
    DBType, NotificationQueryRow, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case, apply_string_filter},
    repository_error::RepositoryError,
    EqualFilter, Pagination, Sort, StringFilter,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type NotificationQuery = NotificationQueryRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NotificationQueryFilter {
    pub id: Option<EqualFilter<String>>,
    pub name: Option<StringFilter>,
    pub reference_name: Option<StringFilter>,
    pub search: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum NotificationQuerySortField {
    Name,
    Id,
}

pub type NotificationQuerySort = Sort<NotificationQuerySortField>;

pub struct NotificationQueryRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationQueryRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationQueryRepository { connection }
    }

    pub fn count(&self, filter: Option<NotificationQueryFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: NotificationQueryFilter,
    ) -> Result<Vec<NotificationQuery>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(
        &self,
        filter: NotificationQueryFilter,
    ) -> Result<Option<NotificationQuery>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<NotificationQueryFilter>,
        sort: Option<NotificationQuerySort>,
    ) -> Result<Vec<NotificationQuery>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                NotificationQuerySortField::Name => {
                    apply_sort_no_case!(query, sort, notification_query_dsl::name);
                }
                NotificationQuerySortField::Id => {
                    apply_sort_no_case!(query, sort, notification_query_dsl::id);
                }
            }
        } else {
            query = query.order(notification_query_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<NotificationQuery>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedQuery = IntoBoxed<'static, notification_query::table, DBType>;

fn create_filtered_query(filter: Option<NotificationQueryFilter>) -> BoxedQuery {
    let mut query = notification_query_dsl::notification_query.into_boxed();

    if let Some(f) = filter {
        let NotificationQueryFilter {
            id,
            name,
            reference_name,
            search,
        } = f;

        apply_equal_filter!(query, id, notification_query_dsl::id);
        apply_string_filter!(query, name, notification_query_dsl::name);
        apply_string_filter!(
            query,
            reference_name,
            notification_query_dsl::reference_name
        );

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(
                notification_query_dsl::name
                    .like(search_term.clone())
                    .or(notification_query_dsl::description.like(search_term.clone()))
                    .or(notification_query_dsl::reference_name.like(search_term.clone()))
                    .or(notification_query_dsl::query.like(search_term.clone())),
            );
        }
    }

    query
}

impl NotificationQueryFilter {
    pub fn new() -> NotificationQueryFilter {
        NotificationQueryFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }
    pub fn name(mut self, filter: StringFilter) -> Self {
        self.name = Some(filter);
        self
    }
    pub fn reference_name(mut self, filter: StringFilter) -> Self {
        self.reference_name = Some(filter);
        self
    }

    pub fn search(mut self, filter: String) -> Self {
        self.search = Some(filter);
        self
    }
}
