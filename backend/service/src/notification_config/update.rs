use super::{
    query::get_notification_config, validate::check_notification_config_exists,
    ModifyNotificationConfigError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    LogType, NotificationConfig, NotificationConfigRow, NotificationConfigRowRepository,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct UpdateNotificationConfig {
    pub id: String,
    pub title: Option<String>,
    pub configuration_data: Option<String>,
    pub parameters: Option<String>,
}

pub fn update_notification_config(
    ctx: &ServiceContext,
    updated_notification_config: UpdateNotificationConfig,
) -> Result<NotificationConfig, ModifyNotificationConfigError> {
    let notification_config = ctx
        .connection
        .transaction_sync(|connection| {
            let notification_config_row = validate(connection, &updated_notification_config)?;
            let updated_notification_config_row =
                generate(updated_notification_config.clone(), notification_config_row)?;
            NotificationConfigRowRepository::new(connection)
                .update_one(&updated_notification_config_row)?;

            get_notification_config(ctx, updated_notification_config_row.id)
                .map_err(ModifyNotificationConfigError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::NotificationConfigUpdated,
        Some(updated_notification_config.id),
        Utc::now().naive_utc(),
    )?;
    Ok(notification_config)
}

pub fn validate(
    connection: &StorageConnection,
    new_notification_config: &UpdateNotificationConfig,
) -> Result<NotificationConfigRow, ModifyNotificationConfigError> {
    let notification_config_row =
        match check_notification_config_exists(&new_notification_config.id, connection)? {
            Some(notification_config_row) => notification_config_row,
            None => return Err(ModifyNotificationConfigError::NotificationConfigDoesNotExist),
        };

    Ok(notification_config_row)
}

pub fn generate(
    UpdateNotificationConfig {
        id: _id, //ID is already used for look up so we can assume it's the same
        title,
        configuration_data,
        parameters,
    }: UpdateNotificationConfig,
    current_notification_config_row: NotificationConfigRow,
) -> Result<NotificationConfigRow, ModifyNotificationConfigError> {
    let mut new_notification_config_row = current_notification_config_row;
    if let Some(title) = title {
        new_notification_config_row.title = title.trim().to_string();
    }
    if let Some(configuration_data) = configuration_data {
        new_notification_config_row.configuration_data = configuration_data;
    }

    if let Some(parameters) = parameters {
        new_notification_config_row.parameters = parameters;
    }

    // Reset the next check datetime in case the schedule has changed, or something needs to be recalculated
    new_notification_config_row.next_check_datetime = None;

    Ok(new_notification_config_row)
}
