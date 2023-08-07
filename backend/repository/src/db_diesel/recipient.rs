use super::{
    recipient_row::{recipient, recipient::dsl as recipient_dsl},
    DBType, NotificationType, RecipientRow, StorageConnection,
};
use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case},
    repository_error::RepositoryError,
    EqualFilter, Pagination, Sort,
};

use diesel::{dsl::IntoBoxed, prelude::*};

pub type Recipient = RecipientRow;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RecipientFilter {
    pub id: Option<EqualFilter<String>>,
    pub notification_type: Option<EqualFilter<NotificationType>>,
    pub search: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum RecipientSortField {
    Name,
    ToAddress,
}

pub type RecipientSort = Sort<RecipientSortField>;

pub struct RecipientRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> RecipientRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        RecipientRepository { connection }
    }

    pub fn count(&self, filter: Option<RecipientFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: RecipientFilter,
    ) -> Result<Vec<Recipient>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query_one(&self, filter: RecipientFilter) -> Result<Option<Recipient>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<RecipientFilter>,
        sort: Option<RecipientSort>,
    ) -> Result<Vec<Recipient>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                RecipientSortField::Name => {
                    apply_sort_no_case!(query, sort, recipient_dsl::name);
                }
                RecipientSortField::ToAddress => {
                    apply_sort_no_case!(query, sort, recipient_dsl::to_address);
                }
            }
        } else {
            query = query.order(recipient_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // // Debug diesel query
        // println!(
        //     "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<Recipient>(&self.connection.connection)?;
        Ok(result)
    }
}

type BoxedRecipientQuery = IntoBoxed<'static, recipient::table, DBType>;

fn create_filtered_query(filter: Option<RecipientFilter>) -> BoxedRecipientQuery {
    let mut query = recipient_dsl::recipient.into_boxed();

    if let Some(f) = filter {
        let RecipientFilter {
            id,
            notification_type,
            search,
        } = f;

        apply_equal_filter!(query, id, recipient_dsl::id);
        apply_equal_filter!(query, notification_type, recipient_dsl::notification_type);

        if let Some(search) = search {
            let search_term = format!("%{}%", search);
            query = query.filter(
                recipient_dsl::name
                    .like(search_term.clone())
                    .or(recipient_dsl::to_address.like(search_term)),
            );
        }
    }

    query
}

impl RecipientFilter {
    pub fn new() -> RecipientFilter {
        RecipientFilter::default()
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }
    pub fn notification_type(mut self, filter: EqualFilter<NotificationType>) -> Self {
        self.notification_type = Some(filter);
        self
    }
    pub fn search(mut self, filter: String) -> Self {
        self.search = Some(filter);
        self
    }
}
