use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use graphql_types::types::NotificationConfigNode;
use service::notification_config::ModifyNotificationConfigError;

mod create;
mod delete;
mod update;

pub use create::*;
pub use delete::*;
pub use update::*;

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
        ModifyNotificationConfigError::NotificationConfigRecipientDoesNotExist => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationConfigError::NotificationConfigRecipientAlreadyExists => {
            BadUserInput(formatted_error)
        }
        ModifyNotificationConfigError::RecipientDoesNotExist => BadUserInput(formatted_error),
        ModifyNotificationConfigError::ModifiedRecordNotFound => InternalError(formatted_error),
        ModifyNotificationConfigError::DatabaseError(_) => InternalError(formatted_error),
        ModifyNotificationConfigError::GenericError(s) => InternalError(s),
    };

    Err(graphql_error.extend())
}
