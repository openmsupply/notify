use super::{recipient_row::recipient::dsl as recipient_dsl, StorageConnection};
use crate::{repository_error::RepositoryError, EqualFilter};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

table! {
    recipient (id) {
        id -> Text,
        name -> Text,
        notification_type -> crate::db_diesel::recipient_row::NotificationTypeMapping,
        to_address -> Text,
    }
}

#[derive(DbEnum, Debug, Clone, PartialEq, Eq, Hash)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum NotificationType {
    Email,
    Telegram,
}

impl Default for NotificationType {
    fn default() -> Self {
        NotificationType::Email
    }
}

impl NotificationType {
    pub fn equal_to(value: NotificationType) -> EqualFilter<NotificationType> {
        EqualFilter {
            equal_to: Some(value),
            not_equal_to: None,
            equal_any: None,
            not_equal_all: None,
            is_null: None,
        }
    }
}

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default)]
#[table_name = "recipient"]
pub struct RecipientRow {
    pub id: String,
    pub name: String,
    pub notification_type: NotificationType,
    pub to_address: String,
}

pub struct RecipientRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientRowRepository { connection }
    }

    pub fn insert_one(&self, row: &RecipientRow) -> Result<(), RepositoryError> {
        diesel::insert_into(recipient_dsl::recipient)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, recipient_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(recipient_dsl::recipient.filter(recipient_dsl::id.eq(recipient_id)))
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<RecipientRow>, RepositoryError> {
        let result = recipient_dsl::recipient
            .filter(recipient_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
