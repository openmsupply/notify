use chrono::{Local, Utc};
use repository::{
    LogType, NotificationConfigRow, NotificationConfigRowRepository, NotificationConfigStatus,
    StorageConnection,
};

use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};

use super::{
    query::{get_notification_config, NotificationConfig},
    validate::check_notification_config_does_not_exist,
    ModifyNotificationConfigError,
};

#[derive(Clone)]
pub struct DuplicateNotificationConfig {
    pub old_id: String,
    pub new_id: String,
}

pub fn duplicate_notification_config(
    ctx: &ServiceContext,
    duplicate_config: DuplicateNotificationConfig,
) -> Result<NotificationConfig, ModifyNotificationConfigError> {
    let notification_config = ctx
        .connection
        .transaction_sync(|connection| {
            let repo = NotificationConfigRowRepository::new(connection);
            let old_config = repo
                .find_one_by_id(&duplicate_config.old_id)?
                .ok_or(ModifyNotificationConfigError::NotificationConfigDoesNotExist)?;

            let new_config_row = generate(&duplicate_config, old_config)?;

            NotificationConfigRowRepository::new(connection).insert_one(&new_config_row)?;

            get_notification_config(ctx, new_config_row.id)
                .map_err(ModifyNotificationConfigError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::NotificationConfigCreated,
        Some(duplicate_config.new_id),
        Utc::now().naive_utc(),
    )?;

    Ok(notification_config)
}

pub fn validate(
    new_config: &DuplicateNotificationConfig,
    connection: &StorageConnection,
) -> Result<(), ModifyNotificationConfigError> {
    if !check_notification_config_does_not_exist(&new_config.new_id, connection)? {
        return Err(ModifyNotificationConfigError::NotificationConfigAlreadyExists);
    }
    Ok(())
}

pub fn generate(
    DuplicateNotificationConfig { old_id: _, new_id }: &DuplicateNotificationConfig,
    old_row: NotificationConfigRow,
) -> Result<NotificationConfigRow, ModifyNotificationConfigError> {
    let new_row = NotificationConfigRow {
        id: new_id.clone(),
        title: format!(
            "{} - {}",
            old_row.title,
            Local::now().format("%Y-%m-%d %H:%M:%S") // There is risk of collision if the user tries to duplicate the same config twice in the same second
        ),
        status: NotificationConfigStatus::Disabled,
        last_run_datetime: None,
        next_due_datetime: None,
        ..old_row
    };

    Ok(new_row)
}
