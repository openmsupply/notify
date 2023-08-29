use super::{recipient_list_row::recipient_list::dsl as recipient_list_dsl, StorageConnection};
use crate::repository_error::RepositoryError;
use diesel::prelude::*;

table! {
    recipient_list (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        sql_query -> Nullable<Text>,
    }
}

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "recipient_list"]
pub struct RecipientListRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sql_query: Option<String>,
}

pub struct RecipientListRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientListRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientListRowRepository { connection }
    }

    pub fn insert_one(&self, row: &RecipientListRow) -> Result<(), RepositoryError> {
        diesel::insert_into(recipient_list_dsl::recipient_list)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(&self, row: &RecipientListRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, recipient_list_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            recipient_list_dsl::recipient_list.filter(recipient_list_dsl::id.eq(recipient_list_id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<RecipientListRow>, RepositoryError> {
        let result = recipient_list_dsl::recipient_list
            .filter(recipient_list_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }
}
