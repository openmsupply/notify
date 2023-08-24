use super::{
    notification_config_recipient_list_row::{
        notification_config_recipient_list,
        notification_config_recipient_list::dsl as notification_config_recipient_list_dsl,
    },
    DBType, StorageConnection,
};
use crate::{
    diesel_macros::apply_equal_filter, repository_error::RepositoryError, EqualFilter,
    NotificationConfigRecipientListRow, Pagination,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type NotificationConfigRecipientList = NotificationConfigRecipientListRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NotificationConfigRecipientListFilter {
    pub id: Option<EqualFilter<String>>,
    pub recipient_list_id: Option<EqualFilter<String>>,
    pub notification_config_id: Option<EqualFilter<String>>,
}

pub struct NotificationConfigRecipientListRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRecipientListRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRecipientListRepository { connection }
    }

    pub fn count(
        &self,
        filter: Option<NotificationConfigRecipientListFilter>,
    ) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: NotificationConfigRecipientListFilter,
    ) -> Result<Vec<NotificationConfigRecipientList>, RepositoryError> {
        self.query(Pagination::new(), Some(filter))
    }

    pub fn query_one(
        &self,
        filter: NotificationConfigRecipientListFilter,
    ) -> Result<Option<NotificationConfigRecipientList>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<NotificationConfigRecipientListFilter>,
    ) -> Result<Vec<NotificationConfigRecipientList>, RepositoryError> {
        let final_query = create_filtered_query(filter)
            .order(notification_config_recipient_list_dsl::id.asc())
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        let result =
            final_query.load::<NotificationConfigRecipientList>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedNotificationConfigRecipientListQuery =
    IntoBoxed<'static, notification_config_recipient_list::table, DBType>;

fn create_filtered_query(
    filter: Option<NotificationConfigRecipientListFilter>,
) -> BoxedNotificationConfigRecipientListQuery {
    let mut query =
        notification_config_recipient_list_dsl::notification_config_recipient_list.into_boxed();

    if let Some(f) = filter {
        let NotificationConfigRecipientListFilter {
            id,
            recipient_list_id,
            notification_config_id,
        } = f;

        apply_equal_filter!(query, id, notification_config_recipient_list_dsl::id);
        apply_equal_filter!(
            query,
            recipient_list_id,
            notification_config_recipient_list_dsl::recipient_list_id
        );
        apply_equal_filter!(
            query,
            notification_config_id,
            notification_config_recipient_list_dsl::notification_config_id
        );
    }

    query
}

impl NotificationConfigRecipientListFilter {
    pub fn new() -> NotificationConfigRecipientListFilter {
        NotificationConfigRecipientListFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn recipient_list_id(mut self, filter: EqualFilter<String>) -> Self {
        self.recipient_list_id = Some(filter);
        self
    }

    pub fn notification_config_id(mut self, filter: EqualFilter<String>) -> Self {
        self.notification_config_id = Some(filter);
        self
    }
}
