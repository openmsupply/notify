use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};

use chrono::Utc;
use repository::{
    LogType, NotificationConfigRecipient, NotificationConfigRecipientRow,
    NotificationConfigRecipientRowRepository, StorageConnection,
};

use super::{validate::check_notification_config_recipient_exists, ModifyNotificationConfigError};

#[derive(Clone)]
pub struct RemoveRecipientFromNotifcationConfig {
    pub recipient_id: String,
    pub notification_config_id: String,
}
pub fn remove_recipient_from_notification_config(
    ctx: &ServiceContext,
    remove_recipient: RemoveRecipientFromNotifcationConfig,
) -> Result<NotificationConfigRecipient, ModifyNotificationConfigError> {
    let config_recipient = ctx
        .connection
        .transaction_sync(|connection| {
            let config_recipient_row = validate(&remove_recipient, connection)?;

            let config_recipient_row_repo =
                NotificationConfigRecipientRowRepository::new(connection);

            match config_recipient_row_repo.delete(&config_recipient_row.id) {
                Ok(_) => {}
                Err(err) => return Err(ModifyNotificationConfigError::from(err)),
            }
            Ok(config_recipient_row)
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::RecipientRemovedFromNotificationConfig,
        Some(config_recipient.recipient_id.clone()),
        Utc::now().naive_utc(),
    )?;

    Ok(config_recipient)
}

pub fn validate(
    remove_recipient: &RemoveRecipientFromNotifcationConfig,
    connection: &StorageConnection,
) -> Result<NotificationConfigRecipientRow, ModifyNotificationConfigError> {
    let config_recipient = match check_notification_config_recipient_exists(
        &remove_recipient.recipient_id,
        &remove_recipient.notification_config_id,
        connection,
    )? {
        Some(config_recipient_row) => config_recipient_row,
        None => return Err(ModifyNotificationConfigError::NotificationConfigRecipientDoesNotExist),
    };

    Ok(config_recipient)
}
