use super::{
    notification_config_recipient_row::{
        notification_config_recipient,
        notification_config_recipient::dsl as notification_config_recipient_dsl,
    },
    DBType, StorageConnection,
};
use crate::{
    diesel_macros::apply_equal_filter, repository_error::RepositoryError, EqualFilter,
    NotificationConfigRecipientRow, Pagination,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type NotificationConfigRecipient = NotificationConfigRecipientRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NotificationConfigRecipientFilter {
    pub id: Option<EqualFilter<String>>,
    pub recipient_id: Option<EqualFilter<String>>,
    pub notification_config_id: Option<EqualFilter<String>>,
}

pub struct NotificationConfigRecipientRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRecipientRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRecipientRepository { connection }
    }

    pub fn count(
        &self,
        filter: Option<NotificationConfigRecipientFilter>,
    ) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: NotificationConfigRecipientFilter,
    ) -> Result<Vec<NotificationConfigRecipient>, RepositoryError> {
        self.query(Pagination::new(), Some(filter))
    }

    pub fn query_one(
        &self,
        filter: NotificationConfigRecipientFilter,
    ) -> Result<Option<NotificationConfigRecipient>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<NotificationConfigRecipientFilter>,
    ) -> Result<Vec<NotificationConfigRecipient>, RepositoryError> {
        let final_query = create_filtered_query(filter)
            .order(notification_config_recipient_dsl::id.asc())
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        let result =
            final_query.load::<NotificationConfigRecipient>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedNotificationConfigRecipientQuery =
    IntoBoxed<'static, notification_config_recipient::table, DBType>;

fn create_filtered_query(
    filter: Option<NotificationConfigRecipientFilter>,
) -> BoxedNotificationConfigRecipientQuery {
    let mut query = notification_config_recipient_dsl::notification_config_recipient.into_boxed();

    if let Some(f) = filter {
        let NotificationConfigRecipientFilter {
            id,
            recipient_id,
            notification_config_id,
        } = f;

        apply_equal_filter!(query, id, notification_config_recipient_dsl::id);
        apply_equal_filter!(
            query,
            recipient_id,
            notification_config_recipient_dsl::recipient_id
        );
        apply_equal_filter!(
            query,
            notification_config_id,
            notification_config_recipient_dsl::notification_config_id
        );
    }

    query
}

impl NotificationConfigRecipientFilter {
    pub fn new() -> NotificationConfigRecipientFilter {
        NotificationConfigRecipientFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn recipient_id(mut self, filter: EqualFilter<String>) -> Self {
        self.recipient_id = Some(filter);
        self
    }

    pub fn notification_config_id(mut self, filter: EqualFilter<String>) -> Self {
        self.notification_config_id = Some(filter);
        self
    }
}
