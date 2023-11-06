use super::{
    notification_event_row::{
        notification_event, notification_event::dsl as notification_event_dsl,
    },
    DBType, NotificationEventRow, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case},
    repository_error::RepositoryError,
    EqualFilter, NotificationEventStatus, Pagination, Sort,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type NotificationEvent = NotificationEventRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NotificationEventFilter {
    pub id: Option<EqualFilter<String>>,
    pub search: Option<String>,
    pub status: Option<EqualFilter<NotificationEventStatus>>,
}

impl NotificationEventFilter {
    pub fn new() -> NotificationEventFilter {
        NotificationEventFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn search(mut self, filter: String) -> Self {
        self.search = Some(filter);
        self
    }
}

#[derive(PartialEq, Debug)]
pub enum NotificationEventSortField {
    Title,
    Id,
    CreatedAt,
    ToAddress,
    Message,
    NotificationType,
    Status,
    ErrorMessage,
}

pub type NotificationEventSort = Sort<NotificationEventSortField>;

pub struct NotificationEventRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationEventRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationEventRepository { connection }
    }

    pub fn count(&self, filter: Option<NotificationEventFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: NotificationEventFilter,
    ) -> Result<Vec<NotificationEvent>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(
        &self,
        filter: NotificationEventFilter,
    ) -> Result<Option<NotificationEvent>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<NotificationEventFilter>,
        sort: Option<NotificationEventSort>,
    ) -> Result<Vec<NotificationEvent>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                NotificationEventSortField::Title => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::title);
                }
                NotificationEventSortField::Id => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::id);
                }
                NotificationEventSortField::CreatedAt => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::created_at);
                }
                NotificationEventSortField::ToAddress => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::to_address);
                }
                NotificationEventSortField::Message => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::message);
                }
                NotificationEventSortField::NotificationType => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::notification_type);
                }
                NotificationEventSortField::Status => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::status);
                }
                NotificationEventSortField::ErrorMessage => {
                    apply_sort_no_case!(query, sort, notification_event_dsl::error_message);
                }
            }
        } else {
            query = query.order(notification_event_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<NotificationEvent>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedQuery = IntoBoxed<'static, notification_event::table, DBType>;

fn create_filtered_query(filter: Option<NotificationEventFilter>) -> BoxedQuery {
    let mut query = notification_event_dsl::notification_event.into_boxed();

    if let Some(f) = filter {
        let NotificationEventFilter { id, search, status } = f;

        apply_equal_filter!(query, id, notification_event_dsl::id);
        apply_equal_filter!(query, status, notification_event_dsl::status);

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(
                notification_event_dsl::title
                    .like(search_term.clone())
                    .or(notification_event_dsl::message.like(search_term.clone()))
                    .or(notification_event_dsl::to_address.like(search_term.clone()))
                    .or(notification_event_dsl::error_message.like(search_term.clone())),
            );
        }
    }

    query
}
