use repository::{
    NotificationConfigRow, NotificationConfigRowRepository, RepositoryError, StorageConnection,
};

pub fn check_notification_config_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<NotificationConfigRow>, RepositoryError> {
    NotificationConfigRowRepository::new(connection).find_one_by_id(id)
}

pub fn check_notification_config_does_not_exist(
    id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let notification_config = check_notification_config_exists(id, connection)?;

    Ok(notification_config.is_none())
}
