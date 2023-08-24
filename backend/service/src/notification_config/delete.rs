use super::validate::check_notification_config_exists;
use crate::service_provider::ServiceContext;
use repository::{
    NotificationConfigRow, NotificationConfigRowRepository, RepositoryError, StorageConnection,
};

#[derive(PartialEq, Debug)]
pub enum DeleteNotificationConfigError {
    NotificationConfigDoesNotExist,
    DatabaseError(RepositoryError),
}

pub fn delete_notification_config(
    ctx: &ServiceContext,
    notification_config_id: &str,
) -> Result<String, DeleteNotificationConfigError> {
    let notification_config = ctx
        .connection
        .transaction_sync(|connection| {
            let notification_config_row = validate(connection, notification_config_id)?;

            let notification_config_repo = NotificationConfigRowRepository::new(connection);
            match notification_config_repo.delete(notification_config_id) {
                Ok(_) => {}
                Err(err) => {
                    return Err(DeleteNotificationConfigError::from(err));
                }
            };
            Ok(notification_config_row)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(notification_config.id)
}

pub fn validate(
    connection: &StorageConnection,
    notification_config_id: &str,
) -> Result<NotificationConfigRow, DeleteNotificationConfigError> {
    let notification_config_row =
        match check_notification_config_exists(notification_config_id, connection)? {
            Some(notification_config_row) => notification_config_row,
            None => return Err(DeleteNotificationConfigError::NotificationConfigDoesNotExist),
        };

    Ok(notification_config_row)
}

impl From<RepositoryError> for DeleteNotificationConfigError {
    fn from(error: RepositoryError) -> Self {
        DeleteNotificationConfigError::DatabaseError(error)
    }
}
