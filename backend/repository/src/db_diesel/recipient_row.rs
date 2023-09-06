use std::str::FromStr;

use super::{recipient_row::recipient::dsl as recipient_dsl, StorageConnection};
use crate::{repository_error::RepositoryError, EqualFilter};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::Serialize;

table! {
    recipient (id) {
        id -> Text,
        name -> Text,
        notification_type -> crate::db_diesel::recipient_row::NotificationTypeMapping,
        to_address -> Text,
        deleted_datetime -> Nullable<Timestamp>,
    }
}

#[derive(DbEnum, Debug, Clone, PartialEq, Eq, Hash, Default, Serialize)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum NotificationType {
    #[default]
    Email,
    Telegram,
    Unknown,
}

impl FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TELEGRAM" => Ok(NotificationType::Telegram),
            "EMAIL" => Ok(NotificationType::Email),
            _ => Ok(NotificationType::Unknown),
        }
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

    pub fn update_one(&self, row: &RecipientRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
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
