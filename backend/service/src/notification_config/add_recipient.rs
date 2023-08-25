use crate::{
    audit_log::audit_log_entry, recipient::validate::check_recipient_exists,
    service_provider::ServiceContext,
};

use chrono::Utc;
use repository::{
    LogType, NotificationConfigRecipient, NotificationConfigRecipientRow,
    NotificationConfigRecipientRowRepository, StorageConnection,
};
use util::uuid::uuid;

use super::{
    validate::{
        check_notification_config_exists, check_notification_config_recipient_does_not_exist,
    },
    ModifyNotificationConfigError,
};

#[derive(Clone)]
pub struct AddRecipientToNotificationConfig {
    pub recipient_id: String,
    pub notification_config_id: String,
}
pub fn add_recipient_to_notification_config(
    ctx: &ServiceContext,
    new_notification_config_recipient: AddRecipientToNotificationConfig,
) -> Result<NotificationConfigRecipient, ModifyNotificationConfigError> {
    let notification_config_recipient = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_notification_config_recipient, connection)?;
            let new_notification_config_recipient_row =
                generate(new_notification_config_recipient.clone())?;
            let repo = NotificationConfigRecipientRowRepository::new(connection);

            repo.insert_one(&new_notification_config_recipient_row)?;

            match repo
                .find_one_by_id(&new_notification_config_recipient_row.id)
                .map_err(ModifyNotificationConfigError::from)?
            {
                Some(config_recipient) => Ok(config_recipient),
                None => Err(ModifyNotificationConfigError::NotificationConfigRecipientDoesNotExist),
            }
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::RecipientAddedToNotificationConfig,
        Some(notification_config_recipient.recipient_id.clone()),
        Utc::now().naive_utc(),
    )?;

    Ok(notification_config_recipient)
}

pub fn validate(
    new_config_recipient: &AddRecipientToNotificationConfig,
    connection: &StorageConnection,
) -> Result<(), ModifyNotificationConfigError> {
    match check_recipient_exists(&new_config_recipient.recipient_id, connection)? {
        Some(_) => (),
        None => return Err(ModifyNotificationConfigError::RecipientDoesNotExist),
    };

    match check_notification_config_exists(
        &new_config_recipient.notification_config_id,
        connection,
    )? {
        Some(_) => (),
        None => return Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist),
    };

    if !check_notification_config_recipient_does_not_exist(
        &new_config_recipient.recipient_id,
        &new_config_recipient.notification_config_id,
        connection,
    )? {
        return Err(ModifyNotificationConfigError::NotificationConfigRecipientAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    AddRecipientToNotificationConfig {
        recipient_id,
        notification_config_id,
    }: AddRecipientToNotificationConfig,
) -> Result<NotificationConfigRecipientRow, ModifyNotificationConfigError> {
    Ok(NotificationConfigRecipientRow {
        id: uuid(),
        recipient_id,
        notification_config_id,
    })
}
