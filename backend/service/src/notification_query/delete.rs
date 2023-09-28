use super::validate::check_notification_query_exists;
use crate::service_provider::ServiceContext;
use repository::{
    RepositoryError, NotificationQueryRow, NotificationQueryRowRepository, StorageConnection,
};

#[derive(PartialEq, Debug)]
pub enum DeleteNotificationQueryError {
    NotificationQueryDoesNotExist,
    DatabaseError(RepositoryError),
}

pub fn delete_notification_query(
    ctx: &ServiceContext,
    notification_query_id: &str,
) -> Result<String, DeleteNotificationQueryError> {
    let notification_query = ctx
        .connection
        .transaction_sync(|connection| {
            let notification_query_row = validate(connection, notification_query_id)?;

            let notification_query_repo = NotificationQueryRowRepository::new(connection);
            match notification_query_repo.delete(notification_query_id) {
                Ok(_) => {}
                Err(err) => {
                    return Err(DeleteNotificationQueryError::from(err));
                }
            };

            Ok(notification_query_row)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(notification_query.id)
}

pub fn validate(
    connection: &StorageConnection,
    notification_query_id: &str,
) -> Result<NotificationQueryRow, DeleteNotificationQueryError> {
    let notification_query_row =
        match check_notification_query_exists(notification_query_id, connection)? {
            Some(notification_query_row) => notification_query_row,
            None => return Err(DeleteNotificationQueryError::NotificationQueryDoesNotExist),
        };

    Ok(notification_query_row)
}

impl From<RepositoryError> for DeleteNotificationQueryError {
    fn from(error: RepositoryError) -> Self {
        DeleteNotificationQueryError::DatabaseError(error)
    }
}
