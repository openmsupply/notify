use super::{
    notification_config_row::notification_config::dsl as notification_config_dsl, StorageConnection,
};
use crate::{repository_error::RepositoryError, EqualFilter};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

table! {
    notification_config (id) {
        id -> Text,
        title -> Text,
        kind -> crate::db_diesel::notification_config_row::NotificationConfigKindMapping,
        configuration_data -> Text,
        parameters -> Text,
        last_check_datetime -> Nullable<Timestamp>,
        next_check_datetime -> Nullable<Timestamp>,
    }
}

#[derive(DbEnum, Debug, Clone, PartialEq, Eq, Hash)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum NotificationConfigKind {
    ColdChain,
    Scheduled,
}

impl Default for NotificationConfigKind {
    fn default() -> Self {
        NotificationConfigKind::ColdChain
    }
}

impl NotificationConfigKind {
    pub fn equal_to(value: NotificationConfigKind) -> EqualFilter<NotificationConfigKind> {
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
#[table_name = "notification_config"]
pub struct NotificationConfigRow {
    pub id: String,
    pub title: String,
    pub kind: NotificationConfigKind,
    // these fields are actually stringified JSON - would be better to store as JSON, however
    // it would appear the diesel JSON types are only available if the postgres feature is enabled...
    pub configuration_data: String,
    pub parameters: String,
    pub last_check_datetime: Option<NaiveDateTime>,
    pub next_check_datetime: Option<NaiveDateTime>,
}

pub struct NotificationConfigRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationConfigRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationConfigRowRepository { connection }
    }

    pub fn insert_one(&self, row: &NotificationConfigRow) -> Result<(), RepositoryError> {
        diesel::insert_into(notification_config_dsl::notification_config)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(&self, row: &NotificationConfigRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            notification_config_dsl::notification_config.filter(notification_config_dsl::id.eq(id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &self,
        id: &str,
    ) -> Result<Option<NotificationConfigRow>, RepositoryError> {
        let result = notification_config_dsl::notification_config
            .filter(notification_config_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_all_due_by_kind(
        &self,
        notification_kind: NotificationConfigKind,
        current_time: NaiveDateTime,
    ) -> Result<Vec<NotificationConfigRow>, RepositoryError> {
        let result = notification_config_dsl::notification_config
            .filter(notification_config_dsl::kind.eq(notification_kind))
            .filter(
                notification_config_dsl::next_check_datetime
                    .is_null()
                    .or(notification_config_dsl::next_check_datetime.le(current_time)),
            )
            .load::<NotificationConfigRow>(&self.connection.connection)?;
        Ok(result)
    }

    pub fn set_last_checked_and_next_check_date(
        &self,
        id: String,
        current_time: NaiveDateTime,
        next_check_time: NaiveDateTime,
    ) -> Result<(), RepositoryError> {
        let query = diesel::update(notification_config_dsl::notification_config)
            .filter(notification_config_dsl::id.eq(id))
            .set((
                notification_config_dsl::last_check_datetime.eq(current_time),
                notification_config_dsl::next_check_datetime.eq(next_check_time),
            ));
        query.execute(&self.connection.connection)?;
        Ok(())
    }
}
