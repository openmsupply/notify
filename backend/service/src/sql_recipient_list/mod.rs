use self::{
    create::{create_sql_recipient_list, CreateSqlRecipientList},
    delete::{delete_sql_recipient_list, DeleteSqlRecipientListError},
    query::{get_sql_recipient_list, get_sql_recipient_lists},
    update::{update_sql_recipient_list, UpdateSqlRecipientList},
};

use super::{ListError, ListResult};
use crate::{service_provider::ServiceContext, SingleRecordError};
use repository::{
    PaginationOption, RepositoryError, SqlRecipientList, SqlRecipientListFilter,
    SqlRecipientListSort,
};

mod tests;

pub mod create;
pub mod delete;
pub mod query;

pub mod update;
pub mod validate;

pub trait SqlRecipientListServiceTrait: Sync + Send {
    fn get_sql_recipient_lists(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<SqlRecipientListFilter>,
        sort: Option<SqlRecipientListSort>,
    ) -> Result<ListResult<SqlRecipientList>, ListError> {
        get_sql_recipient_lists(ctx, pagination, filter, sort)
    }

    fn get_sql_recipient_list(
        &self,
        ctx: &ServiceContext,
        sql_recipient_list_id: String,
    ) -> Result<SqlRecipientList, SingleRecordError> {
        get_sql_recipient_list(ctx, sql_recipient_list_id)
    }

    fn delete_sql_recipient_list(
        &self,
        ctx: &ServiceContext,
        sql_recipient_list_id: &str,
    ) -> Result<String, DeleteSqlRecipientListError> {
        delete_sql_recipient_list(ctx, sql_recipient_list_id)
    }

    fn create_sql_recipient_list(
        &self,
        ctx: &ServiceContext,
        input: CreateSqlRecipientList,
    ) -> Result<SqlRecipientList, ModifySqlRecipientListError> {
        create_sql_recipient_list(ctx, input)
    }

    fn update_sql_recipient_list(
        &self,
        ctx: &ServiceContext,
        input: UpdateSqlRecipientList,
    ) -> Result<SqlRecipientList, ModifySqlRecipientListError> {
        update_sql_recipient_list(ctx, input)
    }
}

pub struct SqlRecipientListService {}
impl SqlRecipientListServiceTrait for SqlRecipientListService {}

#[derive(Debug)]
pub enum ModifySqlRecipientListError {
    SqlRecipientListAlreadyExists,
    ModifiedRecordNotFound,
    DatabaseError(RepositoryError),
    SqlRecipientListDoesNotExist,
    InvalidSqlRecipientListName,
    SqlRecipientListMemberAlreadyExists,
    SqlRecipientListMemberDoesNotExist,
    RecipientDoesNotExist,
    GenericError(String),
}

// PartialEq is only needed for tests
impl PartialEq for ModifySqlRecipientListError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ModifySqlRecipientListError::SqlRecipientListAlreadyExists,
                ModifySqlRecipientListError::SqlRecipientListAlreadyExists,
            ) => true,

            (
                ModifySqlRecipientListError::ModifiedRecordNotFound,
                ModifySqlRecipientListError::ModifiedRecordNotFound,
            ) => true,
            (
                ModifySqlRecipientListError::DatabaseError(self_err),
                ModifySqlRecipientListError::DatabaseError(other_err),
            ) => self_err == other_err,

            (
                ModifySqlRecipientListError::SqlRecipientListDoesNotExist,
                ModifySqlRecipientListError::SqlRecipientListDoesNotExist,
            ) => true,
            (
                ModifySqlRecipientListError::RecipientDoesNotExist,
                ModifySqlRecipientListError::RecipientDoesNotExist,
            ) => true,
            (
                ModifySqlRecipientListError::SqlRecipientListMemberDoesNotExist,
                ModifySqlRecipientListError::SqlRecipientListMemberDoesNotExist,
            ) => true,
            (
                ModifySqlRecipientListError::SqlRecipientListMemberAlreadyExists,
                ModifySqlRecipientListError::SqlRecipientListMemberAlreadyExists,
            ) => true,
            (
                ModifySqlRecipientListError::InvalidSqlRecipientListName,
                ModifySqlRecipientListError::InvalidSqlRecipientListName,
            ) => true,
            _ => false,
        }
    }
}

impl From<RepositoryError> for ModifySqlRecipientListError {
    fn from(err: RepositoryError) -> Self {
        ModifySqlRecipientListError::DatabaseError(err)
    }
}

impl From<SingleRecordError> for ModifySqlRecipientListError {
    fn from(error: SingleRecordError) -> Self {
        use ModifySqlRecipientListError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => ModifiedRecordNotFound,
        }
    }
}
