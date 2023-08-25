use repository::{
    NotificationConfig, NotificationConfigFilter, NotificationConfigRecipient,
    NotificationConfigRecipientList, NotificationConfigSort, PaginationOption, RepositoryError,
};

use crate::{service_provider::ServiceContext, ListError, ListResult, SingleRecordError};

use self::{
    add_recipient::{add_recipient_to_notification_config, AddRecipientToNotificationConfig},
    add_recipient_list::{
        add_recipient_list_to_notification_config, AddRecipientListToNotificationConfig,
    },
    create::{create_notification_config, CreateNotificationConfig},
    delete::{delete_notification_config, DeleteNotificationConfigError},
    query::{get_notification_config, get_notification_configs},
    update::{update_notification_config, UpdateNotificationConfig},
};

mod tests;

pub mod add_recipient;
pub mod add_recipient_list;
pub mod create;
pub mod delete;
pub mod query;
pub mod update;
pub mod validate;

pub trait NotificationConfigServiceTrait: Sync + Send {
    fn get_notification_configs(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<NotificationConfigFilter>,
        sort: Option<NotificationConfigSort>,
    ) -> Result<ListResult<NotificationConfig>, ListError> {
        get_notification_configs(ctx, pagination, filter, sort)
    }

    fn get_notification_config(
        &self,
        ctx: &ServiceContext,
        id: String,
    ) -> Result<NotificationConfig, SingleRecordError> {
        get_notification_config(ctx, id)
    }

    fn create_notification_config(
        &self,
        ctx: &ServiceContext,
        input: CreateNotificationConfig,
    ) -> Result<NotificationConfig, ModifyNotificationConfigError> {
        create_notification_config(ctx, input)
    }

    fn update_notification_config(
        &self,
        ctx: &ServiceContext,
        input: UpdateNotificationConfig,
    ) -> Result<NotificationConfig, ModifyNotificationConfigError> {
        update_notification_config(ctx, input)
    }

    fn add_recipient_to_notification_config(
        &self,
        ctx: &ServiceContext,
        input: AddRecipientToNotificationConfig,
    ) -> Result<NotificationConfigRecipient, ModifyNotificationConfigError> {
        add_recipient_to_notification_config(ctx, input)
    }

    fn add_recipient_list_to_notification_config(
        &self,
        ctx: &ServiceContext,
        input: AddRecipientListToNotificationConfig,
    ) -> Result<NotificationConfigRecipientList, ModifyNotificationConfigError> {
        add_recipient_list_to_notification_config(ctx, input)
    }

    fn delete_notification_config(
        &self,
        ctx: &ServiceContext,
        id: &str,
    ) -> Result<String, DeleteNotificationConfigError> {
        delete_notification_config(ctx, id)
    }
}

pub struct NotificationConfigService {}
impl NotificationConfigServiceTrait for NotificationConfigService {}

#[derive(Debug)]
pub enum ModifyNotificationConfigError {
    NotificationConfigAlreadyExists,
    ModifiedRecordNotFound,
    DatabaseError(RepositoryError),
    NotificationConfigDoesNotExist,
    NotificationConfigRecipientDoesNotExist,
    NotificationConfigRecipientAlreadyExists,
    NotificationConfigRecipientListDoesNotExist,
    NotificationConfigRecipientListAlreadyExists,
    RecipientDoesNotExist,
    RecipientListDoesNotExist,
    GenericError(String),
}

// PartialEq is only needed for tests
impl PartialEq for ModifyNotificationConfigError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ModifyNotificationConfigError::NotificationConfigAlreadyExists,
                ModifyNotificationConfigError::NotificationConfigAlreadyExists,
            ) => true,

            (
                ModifyNotificationConfigError::ModifiedRecordNotFound,
                ModifyNotificationConfigError::ModifiedRecordNotFound,
            ) => true,
            (
                ModifyNotificationConfigError::DatabaseError(self_err),
                ModifyNotificationConfigError::DatabaseError(other_err),
            ) => self_err == other_err,

            (
                ModifyNotificationConfigError::NotificationConfigDoesNotExist,
                ModifyNotificationConfigError::NotificationConfigDoesNotExist,
            ) => true,
            (
                ModifyNotificationConfigError::NotificationConfigRecipientDoesNotExist,
                ModifyNotificationConfigError::NotificationConfigRecipientDoesNotExist,
            ) => true,
            (
                ModifyNotificationConfigError::NotificationConfigRecipientAlreadyExists,
                ModifyNotificationConfigError::NotificationConfigRecipientAlreadyExists,
            ) => true,
            (
                ModifyNotificationConfigError::NotificationConfigRecipientListDoesNotExist,
                ModifyNotificationConfigError::NotificationConfigRecipientListDoesNotExist,
            ) => true,
            (
                ModifyNotificationConfigError::NotificationConfigRecipientListAlreadyExists,
                ModifyNotificationConfigError::NotificationConfigRecipientListAlreadyExists,
            ) => true,
            (
                ModifyNotificationConfigError::RecipientDoesNotExist,
                ModifyNotificationConfigError::RecipientDoesNotExist,
            ) => true,
            (
                ModifyNotificationConfigError::RecipientListDoesNotExist,
                ModifyNotificationConfigError::RecipientListDoesNotExist,
            ) => true,
            _ => false,
        }
    }
}

impl From<RepositoryError> for ModifyNotificationConfigError {
    fn from(err: RepositoryError) -> Self {
        ModifyNotificationConfigError::DatabaseError(err)
    }
}

impl From<SingleRecordError> for ModifyNotificationConfigError {
    fn from(error: SingleRecordError) -> Self {
        use ModifyNotificationConfigError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => ModifiedRecordNotFound,
        }
    }
}
