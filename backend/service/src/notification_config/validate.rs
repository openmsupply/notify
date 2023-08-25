use repository::{
    EqualFilter, NotificationConfigRecipientFilter, NotificationConfigRecipientListFilter,
    NotificationConfigRecipientListRepository, NotificationConfigRecipientListRow,
    NotificationConfigRecipientRepository, NotificationConfigRecipientRow, NotificationConfigRow,
    NotificationConfigRowRepository, RepositoryError, StorageConnection,
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

pub fn check_notification_config_recipient_exists(
    recipient_id: &str,
    notification_config_id: &str,
    connection: &StorageConnection,
) -> Result<Option<NotificationConfigRecipientRow>, RepositoryError> {
    let filter = NotificationConfigRecipientFilter::new()
        .recipient_id(EqualFilter::equal_to(recipient_id))
        .notification_config_id(EqualFilter::equal_to(notification_config_id));

    NotificationConfigRecipientRepository::new(&connection).query_one(filter)
}

pub fn check_notification_config_recipient_does_not_exist(
    recipient_id: &str,
    notification_config_id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let config_recipient = check_notification_config_recipient_exists(
        recipient_id,
        notification_config_id,
        connection,
    )?;

    Ok(config_recipient.is_none())
}

pub fn check_notification_config_recipient_list_exists(
    recipient_list_id: &str,
    notification_config_id: &str,
    connection: &StorageConnection,
) -> Result<Option<NotificationConfigRecipientListRow>, RepositoryError> {
    let filter = NotificationConfigRecipientListFilter::new()
        .recipient_list_id(EqualFilter::equal_to(recipient_list_id))
        .notification_config_id(EqualFilter::equal_to(notification_config_id));

    NotificationConfigRecipientListRepository::new(&connection).query_one(filter)
}

pub fn check_notification_config_recipient_list_does_not_exist(
    recipient_list_id: &str,
    notification_config_id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let config_recipient_list = check_notification_config_recipient_list_exists(
        recipient_list_id,
        notification_config_id,
        connection,
    )?;

    Ok(config_recipient_list.is_none())
}
