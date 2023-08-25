use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};

use chrono::Utc;
use repository::{
    LogType, NotificationConfigRecipientList, NotificationConfigRecipientListRow,
    NotificationConfigRecipientListRowRepository, StorageConnection,
};

use super::{
    validate::check_notification_config_recipient_list_exists, ModifyNotificationConfigError,
};

#[derive(Clone)]
pub struct RemoveRecipientListFromNotifcationConfig {
    pub recipient_list_id: String,
    pub notification_config_id: String,
}
pub fn remove_recipient_list_from_notification_config(
    ctx: &ServiceContext,
    remove_recipient_list: RemoveRecipientListFromNotifcationConfig,
) -> Result<NotificationConfigRecipientList, ModifyNotificationConfigError> {
    let config_recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            let config_recipient_list_row = validate(&remove_recipient_list, connection)?;

            let config_recipient_list_row_repo =
                NotificationConfigRecipientListRowRepository::new(connection);

            match config_recipient_list_row_repo.delete(&config_recipient_list_row.id) {
                Ok(_) => {}
                Err(err) => return Err(ModifyNotificationConfigError::from(err)),
            }
            Ok(config_recipient_list_row)
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::RecipientListRemovedFromNotificationConfig,
        Some(config_recipient_list.recipient_list_id.clone()),
        Utc::now().naive_utc(),
    )?;

    Ok(config_recipient_list)
}

pub fn validate(
    remove_recipient_list: &RemoveRecipientListFromNotifcationConfig,
    connection: &StorageConnection,
) -> Result<NotificationConfigRecipientListRow, ModifyNotificationConfigError> {
    let config_recipient_list = match check_notification_config_recipient_list_exists(
        &remove_recipient_list.recipient_list_id,
        &remove_recipient_list.notification_config_id,
        connection,
    )? {
        Some(config_recipient_list_row) => config_recipient_list_row,
        None => {
            return Err(ModifyNotificationConfigError::NotificationConfigRecipientListDoesNotExist)
        }
    };

    Ok(config_recipient_list)
}
