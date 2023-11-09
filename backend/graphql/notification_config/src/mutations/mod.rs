use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use graphql_types::types::NotificationConfigNode;
use service::notification_config::ModifyNotificationConfigError;

mod create;
mod delete;
mod update;
mod duplicate;

pub use create::*;
pub use delete::*;
pub use update::*;
pub use duplicate::*;

#[derive(Union)]
pub enum ModifyNotificationConfigResponse {
    Response(NotificationConfigNode),
}

pub fn map_error(error: ModifyNotificationConfigError) -> Result<ModifyNotificationConfigResponse> {
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ModifyNotificationConfigError::NotificationConfigDoesNotExist => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationConfigError::NotificationConfigAlreadyExists => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationConfigError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyNotificationConfigError::DatabaseError(_) => InternalError(formatted_error),
        ModifyNotificationConfigError::InternalError(s) => InternalError(s),
        ModifyNotificationConfigError::BadUserInput(s) => BadUserInput(s),
    };

    Err(graphql_error.extend())
}
