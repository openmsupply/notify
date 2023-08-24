use super::{
    notification_config_recipient_list_row::notification_config_recipient_list::dsl as notification_config_recipient_list_dsl,
    StorageConnection,
};
use crate::repository_error::RepositoryError;
use diesel::prelude::*;

table! {
    notification_config_recipient_list (id) {
        id -> Text,
        recipient_list_id -> Text,
        notification_config_id -> Text,
    }
}

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "notification_config_recipient_list"]
pub struct NotificationConfigRecipientListRow {
    pub id: String,
    pub recipient_list_id: String,
    pub notification_config_id: String,
}

pub struct NotificationConfigRecipientListRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRecipientListRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRecipientListRowRepository { connection }
    }

    pub fn insert_one(
        &self,
        row: &NotificationConfigRecipientListRow,
    ) -> Result<(), RepositoryError> {
        diesel::insert_into(
            notification_config_recipient_list_dsl::notification_config_recipient_list,
        )
        .values(row)
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(
        &self,
        row: &NotificationConfigRecipientListRow,
    ) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            notification_config_recipient_list_dsl::notification_config_recipient_list
                .filter(notification_config_recipient_list_dsl::id.eq(id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &self,
        id: &str,
    ) -> Result<Option<NotificationConfigRecipientListRow>, RepositoryError> {
        let result = notification_config_recipient_list_dsl::notification_config_recipient_list
            .filter(notification_config_recipient_list_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
