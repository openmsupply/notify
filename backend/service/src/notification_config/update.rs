use super::{
    query::{get_notification_config, NotificationConfig},
    validate::check_notification_config_exists,
    ModifyNotificationConfigError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    LogType, NotificationConfigRow, NotificationConfigRowRepository, NotificationConfigStatus,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct UpdateNotificationConfig {
    pub id: String,
    pub title: Option<String>,
    pub configuration_data: Option<String>,
    pub status: Option<NotificationConfigStatus>,
    pub parameters: Option<String>,
    pub recipient_ids: Option<Vec<String>>,
    pub recipient_list_ids: Option<Vec<String>>,
    pub sql_recipient_list_ids: Option<Vec<String>>,
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
        status,
        parameters,
        recipient_ids,
        recipient_list_ids,
        sql_recipient_list_ids,
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

    if let Some(status) = status {
        new_notification_config_row.status = status;
    }

    if let Some(parameters) = parameters {
        new_notification_config_row.parameters = parameters;
    }

    if let Some(recipient_ids) = recipient_ids {
        let recipient_json = serde_json::to_string(&recipient_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_notification_config_row.recipient_ids = recipient_json;
    }

    if let Some(recipient_list_ids) = recipient_list_ids {
        let recipient_json = serde_json::to_string(&recipient_list_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_notification_config_row.recipient_list_ids = recipient_json;
    }

    if let Some(sql_recipient_list_ids) = sql_recipient_list_ids {
        let recipient_json = serde_json::to_string(&sql_recipient_list_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_notification_config_row.sql_recipient_list_ids = recipient_json;
    }

    // Reset the next check datetime in case the schedule has changed, or something needs to be recalculated
    new_notification_config_row.next_due_datetime = None;

    Ok(new_notification_config_row)
}
