use self::{
    create::{create_recipient, CreateRecipient},
    delete::{delete_recipient, DeleteRecipientError},
    query::{get_recipient, get_recipients},
    update::{update_recipient, UpdateRecipient},
};

use super::{ListError, ListResult};
use crate::{service_provider::ServiceContext, SingleRecordError};
use repository::{PaginationOption, Recipient, RecipientFilter, RecipientSort, RepositoryError};

mod tests;

pub mod create;
pub mod delete;
pub mod query;
pub mod telegram;
pub mod update;
pub mod validate;

pub trait RecipientServiceTrait: Sync + Send {
    fn get_recipients(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<RecipientFilter>,
        sort: Option<RecipientSort>,
    ) -> Result<ListResult<Recipient>, ListError> {
        get_recipients(ctx, pagination, filter, sort)
    }

    fn get_recipient(
        &self,
        ctx: &ServiceContext,
        recipient_id: String,
    ) -> Result<Recipient, SingleRecordError> {
        get_recipient(ctx, recipient_id)
    }

    fn delete_recipient(
        &self,
        ctx: &ServiceContext,
        recipient_id: &str,
    ) -> Result<String, DeleteRecipientError> {
        delete_recipient(ctx, recipient_id)
    }

    fn create_recipient(
        &self,
        ctx: &ServiceContext,
        input: CreateRecipient,
    ) -> Result<Recipient, ModifyRecipientError> {
        create_recipient(ctx, input)
    }

    fn update_recipient(
        &self,
        ctx: &ServiceContext,
        input: UpdateRecipient,
    ) -> Result<Recipient, ModifyRecipientError> {
        update_recipient(ctx, input)
    }
}

pub struct RecipientService {}
impl RecipientServiceTrait for RecipientService {}

#[derive(Debug, PartialEq)]
pub enum ModifyRecipientError {
    RecipientAlreadyExists,
    ModifiedRecordNotFound,
    DatabaseError(RepositoryError),
    RecipientDoesNotExist,
    GenericError(String),
}

impl From<RepositoryError> for ModifyRecipientError {
    fn from(err: RepositoryError) -> Self {
        ModifyRecipientError::DatabaseError(err)
    }
}

impl From<SingleRecordError> for ModifyRecipientError {
    fn from(error: SingleRecordError) -> Self {
        use ModifyRecipientError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => ModifiedRecordNotFound,
        }
    }
}
