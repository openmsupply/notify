use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use service::notification_query::ModifyNotificationQueryError;

mod create;
mod delete;
mod update;

pub use create::*;
pub use delete::*;
pub use update::*;

use crate::types::NotificationQueryNode;

#[derive(Union)]
pub enum ModifyNotificationQueryResponse {
    Response(NotificationQueryNode),
}

pub fn map_error(error: ModifyNotificationQueryError) -> Result<ModifyNotificationQueryResponse> {
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyNotificationQueryError::NotificationQueryDoesNotExist => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationQueryError::NotificationQueryAlreadyExists => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationQueryError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyNotificationQueryError::DatabaseError(_) => InternalError(formatted_error),
        ModifyNotificationQueryError::InternalError(s) => InternalError(s),
        ModifyNotificationQueryError::InvalidNotificationQueryName => BadUserInput(formatted_error),
        ModifyNotificationQueryError::BadUserInput(s) => BadUserInput(s),
    };

    Err(graphql_error.extend())
}
