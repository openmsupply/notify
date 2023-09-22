use super::{
    notification_config_row::{
        notification_config, notification_config::dsl as notification_config_dsl,
    },
    DBType, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case, apply_string_filter},
    repository_error::RepositoryError,
    EqualFilter, NotificationConfigKind, NotificationConfigRow, Pagination, Sort, StringFilter, NotificationConfigStatus, 
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type NotificationConfig = NotificationConfigRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NotificationConfigFilter {
    pub id: Option<EqualFilter<String>>,
    pub kind: Option<EqualFilter<NotificationConfigKind>>,
    pub title: Option<StringFilter>,
    pub search: Option<String>,
    pub status: Option<EqualFilter<NotificationConfigStatus>>,
}

#[derive(PartialEq, Debug)]
pub enum NotificationConfigSortField {
    Title,
}

pub type NotificationConfigSort = Sort<NotificationConfigSortField>;

pub struct NotificationConfigRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRepository { connection }
    }

    pub fn count(&self, filter: Option<NotificationConfigFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: NotificationConfigFilter,
    ) -> Result<Vec<NotificationConfig>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(
        &self,
        filter: NotificationConfigFilter,
    ) -> Result<Option<NotificationConfig>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<NotificationConfigFilter>,
        sort: Option<NotificationConfigSort>,
    ) -> Result<Vec<NotificationConfig>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                NotificationConfigSortField::Title => {
                    apply_sort_no_case!(query, sort, notification_config::title);
                }
            }
        } else {
            query = query.order(notification_config::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        let result = final_query.load::<NotificationConfig>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedNotificationConfigQuery = IntoBoxed<'static, notification_config::table, DBType>;

fn create_filtered_query(filter: Option<NotificationConfigFilter>) -> BoxedNotificationConfigQuery {
    let mut query = notification_config_dsl::notification_config.into_boxed();

    if let Some(f) = filter {
        let NotificationConfigFilter {
            id,
            kind,
            title,
            search,
            status,
        } = f;

        apply_equal_filter!(query, id, notification_config_dsl::id);
        apply_equal_filter!(query, kind, notification_config_dsl::kind);
        apply_string_filter!(query, title, notification_config_dsl::title);
        apply_equal_filter!(query, status, notification_config_dsl::status);

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(notification_config_dsl::title.like(search_term.clone()));
        }
    }

    query
}

impl NotificationConfigFilter {
    pub fn new() -> NotificationConfigFilter {
        NotificationConfigFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }
    pub fn kind(mut self, filter: EqualFilter<NotificationConfigKind>) -> Self {
        self.kind = Some(filter);
        self
    }
    pub fn title(mut self, filter: StringFilter) -> Self {
        self.title = Some(filter);
        self
    }
    pub fn search(mut self, filter: String) -> Self {
        self.search = Some(filter);
        self
    }
    pub fn status(mut self, filter: EqualFilter<NotificationConfigStatus>) -> Self {
        self.status = Some(filter);
        self
    }
}
