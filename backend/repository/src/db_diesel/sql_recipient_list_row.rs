use super::{
    sql_recipient_list_row::sql_recipient_list::dsl as sql_recipient_list_dsl, StorageConnection,
};
use crate::repository_error::RepositoryError;
use diesel::prelude::*;

/*
CREATE TABLE
    sql_recipient_list (
        id TEXT NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        query TEXT NOT NULL,
        required_parameters TEXT NOT NULL, -- JSON e.g. {"region":"string","tags":"string[]", "limit": "number"}
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL
    );

*/

table! {
    sql_recipient_list (id) {
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
#[table_name = "sql_recipient_list"]
pub struct SqlRecipientListRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub required_parameters: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct SqlRecipientListRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> SqlRecipientListRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        SqlRecipientListRowRepository { connection }
    }

    pub fn insert_one(&self, row: &SqlRecipientListRow) -> Result<(), RepositoryError> {
        diesel::insert_into(sql_recipient_list_dsl::sql_recipient_list)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(&self, row: &SqlRecipientListRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, sql_recipient_list_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            sql_recipient_list_dsl::sql_recipient_list
                .filter(sql_recipient_list_dsl::id.eq(sql_recipient_list_id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<SqlRecipientListRow>, RepositoryError> {
        let result = sql_recipient_list_dsl::sql_recipient_list
            .filter(sql_recipient_list_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
