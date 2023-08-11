use super::{
    recipient_list_member_row::{
        recipient_list_member, recipient_list_member::dsl as recipient_list_member_dsl,
    },
    DBType, RecipientListMemberRow, StorageConnection,
};
use crate::{
    diesel_macros::apply_equal_filter, repository_error::RepositoryError, EqualFilter, Pagination,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type RecipientListMember = RecipientListMemberRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RecipientListMemberFilter {
    pub id: Option<EqualFilter<String>>,
    pub recipient_id: Option<EqualFilter<String>>,
    pub recipient_list_id: Option<EqualFilter<String>>,
}

pub struct RecipientListMemberRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientListMemberRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientListMemberRepository { connection }
    }

    pub fn count(&self, filter: Option<RecipientListMemberFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: RecipientListMemberFilter,
    ) -> Result<Vec<RecipientListMember>, RepositoryError> {
        self.query(Pagination::new(), Some(filter))
    }

    pub fn query_one(
        &self,
        filter: RecipientListMemberFilter,
    ) -> Result<Option<RecipientListMember>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<RecipientListMemberFilter>,
    ) -> Result<Vec<RecipientListMember>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        query = query.order(recipient_list_member_dsl::id.asc());

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<RecipientListMember>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedRecipientQuery = IntoBoxed<'static, recipient_list_member::table, DBType>;

fn create_filtered_query(filter: Option<RecipientListMemberFilter>) -> BoxedRecipientQuery {
    let mut query = recipient_list_member_dsl::recipient_list_member.into_boxed();

    if let Some(f) = filter {
        let RecipientListMemberFilter {
            id,
            recipient_id,
            recipient_list_id,
        } = f;

        apply_equal_filter!(query, id, recipient_list_member_dsl::id);
        apply_equal_filter!(query, recipient_id, recipient_list_member_dsl::recipient_id);
        apply_equal_filter!(
            query,
            recipient_list_id,
            recipient_list_member_dsl::recipient_list_id
        );
    }

    query
}

impl RecipientListMemberFilter {
    pub fn new() -> RecipientListMemberFilter {
        RecipientListMemberFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }
    pub fn recipient_id(mut self, filter: EqualFilter<String>) -> Self {
        self.recipient_id = Some(filter);
        self
    }
    pub fn recipient_list_id(mut self, filter: EqualFilter<String>) -> Self {
        self.recipient_list_id = Some(filter);
        self
    }
}
