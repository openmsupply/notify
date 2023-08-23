use super::{recipient_row::recipient::dsl as recipient_dsl, StorageConnection};
use crate::{repository_error::RepositoryError, EqualFilter, RowRepository};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

table! {
    recipient (id) {
        id -> Text,
        name -> Text,
        notification_type -> crate::db_diesel::recipient_row::NotificationTypeMapping,
        to_address -> Text,
        deleted_datetime -> Nullable<Timestamp>,
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

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "recipient"]
pub struct RecipientRow {
    pub id: String,
    pub name: String,
    pub notification_type: NotificationType,
    pub to_address: String,
    pub deleted_datetime: Option<NaiveDateTime>,
}

pub struct RecipientRowRepository<'a> {
    connection: &'a StorageConnection,
    repository: RowRepository<'a, recipient_dsl::recipient>,
}

impl<'a> RecipientRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientRowRepository {
            connection,
            repository: RowRepository::new(connection, recipient_dsl::recipient),
        }
    }

    pub fn insert_one(&self, row: &RecipientRow) -> Result<(), RepositoryError> {
        self.repository.insert_one(row)
    }

    pub fn update_one(&self, row: &RecipientRow) -> Result<(), RepositoryError> {
        self.repository.update_one(row)
    }

    pub fn delete(&self, recipient_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(recipient_dsl::recipient.filter(recipient_dsl::id.eq(recipient_id)))
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn mark_deleted(&self, id: &str) -> Result<(), RepositoryError> {
        let query = diesel::update(recipient_dsl::recipient)
            .filter(recipient_dsl::id.eq(id))
            .filter(recipient_dsl::deleted_datetime.is_null())
            .set(recipient_dsl::deleted_datetime.eq(chrono::Utc::now().naive_utc()));
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<RecipientRow>, RepositoryError> {
        let result = recipient_dsl::recipient
            .filter(recipient_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
    pub fn find_one_by_to_address_and_type(
        &self,
        address: &str,
        notification_type: NotificationType,
    ) -> Result<Option<RecipientRow>, RepositoryError> {
        let result = recipient_dsl::recipient
            .filter(recipient_dsl::to_address.eq(address))
            .filter(recipient_dsl::notification_type.eq(notification_type))
            .filter(recipient_dsl::deleted_datetime.is_null())
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
