use super::{
    recipient_list_member_row::recipient_list_member::dsl as recipient_list_member_dsl,
    StorageConnection,
};
use crate::repository_error::RepositoryError;
use diesel::prelude::*;

table! {
    recipient_list_member (id) {
        id -> Text,
        recipient_id -> Text,
        recipient_list_id -> Text,
    }
}

#[derive(
    Clone, Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Eq, Default,
)]
#[table_name = "recipient_list_member"]
pub struct RecipientListMemberRow {
    pub id: String,
    pub recipient_id: String,
    pub recipient_list_id: String,
}

pub struct RecipientListMemberRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientListMemberRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientListMemberRowRepository { connection }
    }

    pub fn insert_one(&self, row: &RecipientListMemberRow) -> Result<(), RepositoryError> {
        diesel::insert_into(recipient_list_member_dsl::recipient_list_member)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn update_one(&self, row: &RecipientListMemberRow) -> Result<(), RepositoryError> {
        let query = diesel::update(row).set(row);
        // println!("{}", diesel::debug_query::<DBType, _>(&query).to_string());
        query.execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete(&self, recipient_list_member_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(
            recipient_list_member_dsl::recipient_list_member
                .filter(recipient_list_member_dsl::id.eq(recipient_list_member_id)),
        )
        .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &self,
        id: &str,
    ) -> Result<Option<RecipientListMemberRow>, RepositoryError> {
        let result = recipient_list_member_dsl::recipient_list_member
            .filter(recipient_list_member_dsl::id.eq(id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    // TODO: delete all by list id
}
