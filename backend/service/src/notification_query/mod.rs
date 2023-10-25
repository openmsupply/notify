use self::{
    create::{create_notification_query, CreateNotificationQuery},
    delete::{delete_notification_query, DeleteNotificationQueryError},
    query::{get_notification_queries, get_notification_query},
    update::{update_notification_query, UpdateNotificationQuery},
};

use super::{ListError, ListResult};
use crate::{service_provider::ServiceContext, SingleRecordError};

use repository::{
    NotificationQuery, NotificationQueryFilter, NotificationQuerySort, PaginationOption,
    RepositoryError,
};

mod tests;

pub mod create;
pub mod delete;
pub mod query;

pub mod update;
pub mod validate;

pub trait NotificationQueryServiceTrait: Sync + Send {
    fn get_notification_queries(
        &self,
        ctx: &ServiceContext,
        pagination: Option<PaginationOption>,
        filter: Option<NotificationQueryFilter>,
        sort: Option<NotificationQuerySort>,
    ) -> Result<ListResult<NotificationQuery>, ListError> {
        get_notification_queries(ctx, pagination, filter, sort)
    }

    fn get_notification_query(
        &self,
        ctx: &ServiceContext,
        notification_query_id: String,
    ) -> Result<NotificationQuery, SingleRecordError> {
        get_notification_query(ctx, notification_query_id)
    }

    fn delete_notification_query(
        &self,
        ctx: &ServiceContext,
        notification_query_id: &str,
    ) -> Result<String, DeleteNotificationQueryError> {
        delete_notification_query(ctx, notification_query_id)
    }

    fn create_notification_query(
        &self,
        ctx: &ServiceContext,
        input: CreateNotificationQuery,
    ) -> Result<NotificationQuery, ModifyNotificationQueryError> {
        create_notification_query(ctx, input)
    }

    fn update_notification_query(
        &self,
        ctx: &ServiceContext,
        input: UpdateNotificationQuery,
    ) -> Result<NotificationQuery, ModifyNotificationQueryError> {
        update_notification_query(ctx, input)
    }
}

pub struct NotificationQueryService {}
impl NotificationQueryServiceTrait for NotificationQueryService {}

#[derive(Debug, PartialEq)]
pub enum ModifyNotificationQueryError {
    NotificationQueryAlreadyExists,
    ReferenceNameAlreadyExists,
    ModifiedRecordNotFound,
    DatabaseError(RepositoryError),
    NotificationQueryDoesNotExist,
    InvalidNotificationQueryName,
    InternalError(String),
    BadUserInput(String),
}
impl From<RepositoryError> for ModifyNotificationQueryError {
    fn from(err: RepositoryError) -> Self {
        ModifyNotificationQueryError::DatabaseError(err)
    }
}

impl From<SingleRecordError> for ModifyNotificationQueryError {
    fn from(error: SingleRecordError) -> Self {
        use ModifyNotificationQueryError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => ModifiedRecordNotFound,
        }
    }
}
