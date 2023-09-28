use super::{
    notification_query_row::notification_query::dsl as notification_query_dsl, StorageConnection,
};
use crate::repository_error::RepositoryError;
use chrono::NaiveDateTime;
use diesel::prelude::*;

table! {
    notification_query (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        query -> Text,
        required_parameters -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "notification_query"]
pub struct NotificationQueryRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub required_parameters: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NotificationQueryRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> NotificationQueryRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        NotificationQueryRowRepository { connection }
    }

    pub fn insert_one(&self, row: &NotificationQueryRow) -> Result<(), RepositoryError> {
        let query = diesel::insert_into(notification_query_dsl::notification_query).values(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(&self, row: &NotificationQueryRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, notification_query_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            notification_query_dsl::notification_query
                .filter(notification_query_dsl::id.eq(notification_query_id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &self,
        id: &str,
    ) -> Result<Option<NotificationQueryRow>, RepositoryError> {
        let result = notification_query_dsl::notification_query
            .filter(notification_query_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
