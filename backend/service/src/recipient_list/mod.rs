use self::{
    create::{create_recipient_list, CreateRecipientList},
    delete::{delete_recipient_list, DeleteRecipientListError},
    query::{get_recipient_list, get_recipient_lists},
    update::{update_recipient_list, UpdateRecipientList},
};

use super::{ListError, ListResult};
use crate::{service_provider::ServiceContext, SingleRecordError};
use repository::{
    PaginationOption, RecipientList, RecipientListFilter, RecipientListSort, RepositoryError,
};

mod tests;

pub mod create;
pub mod delete;
pub mod query;
pub mod update;
pub mod validate;

pub trait RecipientListServiceTrait: Sync + Send {
    fn get_recipient_lists(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<RecipientListFilter>,
        sort: Option<RecipientListSort>,
    ) -> Result<ListResult<RecipientList>, ListError> {
        get_recipient_lists(ctx, pagination, filter, sort)
    }

    fn get_recipient_list(
        &self,
        ctx: &ServiceContext,
        recipient_list_id: String,
    ) -> Result<RecipientList, SingleRecordError> {
        get_recipient_list(ctx, recipient_list_id)
    }

    fn delete_recipient_list(
        &self,
        ctx: &ServiceContext,
        recipient_list_id: &str,
    ) -> Result<String, DeleteRecipientListError> {
        delete_recipient_list(ctx, recipient_list_id)
    }

    fn create_recipient_list(
        &self,
        ctx: &ServiceContext,
        input: CreateRecipientList,
    ) -> Result<RecipientList, ModifyRecipientListError> {
        create_recipient_list(ctx, input)
    }

    fn update_recipient_list(
        &self,
        ctx: &ServiceContext,
        input: UpdateRecipientList,
    ) -> Result<RecipientList, ModifyRecipientListError> {
        update_recipient_list(ctx, input)
    }
}

pub struct RecipientListService {}
impl RecipientListServiceTrait for RecipientListService {}

#[derive(Debug)]
pub enum ModifyRecipientListError {
    RecipientListAlreadyExists,
    ModifiedRecordNotFound,
    DatabaseError(RepositoryError),
    RecipientListDoesNotExist,
    GenericError(String),
}

// PartialEq is only needed for tests
impl PartialEq for ModifyRecipientListError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ModifyRecipientListError::RecipientListAlreadyExists,
                ModifyRecipientListError::RecipientListAlreadyExists,
            ) => true,

            (
                ModifyRecipientListError::ModifiedRecordNotFound,
                ModifyRecipientListError::ModifiedRecordNotFound,
            ) => true,
            (
                ModifyRecipientListError::DatabaseError(self_err),
                ModifyRecipientListError::DatabaseError(other_err),
            ) => self_err == other_err,

            (
                ModifyRecipientListError::RecipientListDoesNotExist,
                ModifyRecipientListError::RecipientListDoesNotExist,
            ) => true,
            _ => false,
        }
    }
}

impl From<RepositoryError> for ModifyRecipientListError {
    fn from(err: RepositoryError) -> Self {
        ModifyRecipientListError::DatabaseError(err)
    }
}

impl From<SingleRecordError> for ModifyRecipientListError {
    fn from(error: SingleRecordError) -> Self {
        use ModifyRecipientListError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => ModifiedRecordNotFound,
        }
    }
}
