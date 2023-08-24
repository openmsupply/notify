use super::{
    notification_config_recipient_row::notification_config_recipient::dsl as notification_config_recipient_dsl,
    StorageConnection,
};
use crate::repository_error::RepositoryError;
use diesel::prelude::*;

table! {
    notification_config_recipient (id) {
        id -> Text,
        recipient_id -> Text,
        notification_config_id -> Text,
    }
}

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "notification_config_recipient"]
pub struct NotificationConfigRecipientRow {
    pub id: String,
    pub recipient_id: String,
    pub notification_config_id: String,
}

pub struct NotificationConfigRecipientRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRecipientRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRecipientRowRepository { connection }
    }

    pub fn insert_one(&self, row: &NotificationConfigRecipientRow) -> Result<(), RepositoryError> {
        diesel::insert_into(notification_config_recipient_dsl::notification_config_recipient)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            notification_config_recipient_dsl::notification_config_recipient
                .filter(notification_config_recipient_dsl::id.eq(id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &self,
        id: &str,
    ) -> Result<Option<NotificationConfigRecipientRow>, RepositoryError> {
        let result = notification_config_recipient_dsl::notification_config_recipient
            .filter(notification_config_recipient_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
