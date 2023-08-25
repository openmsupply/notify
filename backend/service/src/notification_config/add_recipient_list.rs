use crate::{
    audit_log::audit_log_entry, recipient_list::validate::check_recipient_list_exists,
    service_provider::ServiceContext,
};

use chrono::Utc;
use repository::{
    LogType, NotificationConfigRecipientList, NotificationConfigRecipientListRow,
    NotificationConfigRecipientListRowRepository, StorageConnection,
};
use util::uuid::uuid;

use super::{
    validate::{
        check_notification_config_exists, check_notification_config_recipient_list_does_not_exist,
    },
    ModifyNotificationConfigError,
};

#[derive(Clone)]
pub struct AddRecipientListToNotificationConfig {
    pub recipient_list_id: String,
    pub notification_config_id: String,
}
pub fn add_recipient_list_to_notification_config(
    ctx: &ServiceContext,
    new_notification_config_recipient_list: AddRecipientListToNotificationConfig,
) -> Result<NotificationConfigRecipientList, ModifyNotificationConfigError> {
    let notification_config_recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_notification_config_recipient_list, connection)?;
            let new_notification_config_recipient_list_row =
                generate(new_notification_config_recipient_list.clone())?;
            let repo = NotificationConfigRecipientListRowRepository::new(connection);

            repo.insert_one(&new_notification_config_recipient_list_row)?;

            match repo
                .find_one_by_id(&new_notification_config_recipient_list_row.id)
                .map_err(ModifyNotificationConfigError::from)?
            {
                Some(config_recipient_list) => Ok(config_recipient_list),
                None => {
                    Err(ModifyNotificationConfigError::NotificationConfigRecipientListDoesNotExist)
                }
            }
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::RecipientListAddedToNotificationConfig,
        Some(notification_config_recipient_list.recipient_list_id.clone()),
        Utc::now().naive_utc(),
    )?;

    Ok(notification_config_recipient_list)
}

pub fn validate(
    new_config_recipient_list: &AddRecipientListToNotificationConfig,
    connection: &StorageConnection,
) -> Result<(), ModifyNotificationConfigError> {
    match check_recipient_list_exists(&new_config_recipient_list.recipient_list_id, connection)? {
        Some(_) => (),
        None => return Err(ModifyNotificationConfigError::RecipientListDoesNotExist),
    };

    match check_notification_config_exists(
        &new_config_recipient_list.notification_config_id,
        connection,
    )? {
        Some(_) => (),
        None => return Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist),
    };

    if !check_notification_config_recipient_list_does_not_exist(
        &new_config_recipient_list.recipient_list_id,
        &new_config_recipient_list.notification_config_id,
        connection,
    )? {
        return Err(ModifyNotificationConfigError::NotificationConfigRecipientListAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    AddRecipientListToNotificationConfig {
        recipient_list_id,
        notification_config_id,
    }: AddRecipientListToNotificationConfig,
) -> Result<NotificationConfigRecipientListRow, ModifyNotificationConfigError> {
    Ok(NotificationConfigRecipientListRow {
        id: uuid(),
        recipient_list_id,
        notification_config_id,
    })
}
