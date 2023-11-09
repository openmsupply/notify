use chrono::Utc;
use repository::{
    LogType, NotificationConfigKind, NotificationConfigRow, NotificationConfigRowRepository,
    NotificationConfigStatus, StorageConnection,
};

use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};

use super::{
    query::{get_notification_config, NotificationConfig},
    validate::check_notification_config_does_not_exist,
    ModifyNotificationConfigError,
};

#[derive(Clone)]
pub struct DuplicateNotificationConfig {
    pub id: String,
    pub title: String,
    pub kind: NotificationConfigKind,
    pub status: NotificationConfigStatus,
    pub configuration_data: Option<String>,
    pub parameters: Option<String>,
    pub recipient_ids: Option<Vec<String>>,
    pub recipient_list_ids: Option<Vec<String>>,
    pub sql_recipient_list_ids: Option<Vec<String>>,
}

pub fn duplicate_notification_config(
    ctx: &ServiceContext,
    new_config: DuplicateNotificationConfig,
) -> Result<NotificationConfig, ModifyNotificationConfigError> {
    let notification_config = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_config, connection)?;
            let new_config_row = generate(new_config.clone())?;

            NotificationConfigRowRepository::new(connection).insert_one(&new_config_row)?;

            get_notification_config(ctx, new_config_row.id)
                .map_err(ModifyNotificationConfigError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::NotificationConfigCreated,
        Some(new_config.id),
        Utc::now().naive_utc(),
    )?;

    Ok(notification_config)
}

pub fn validate(
    new_config: &DuplicateNotificationConfig,
    connection: &StorageConnection,
) -> Result<(), ModifyNotificationConfigError> {
    if !check_notification_config_does_not_exist(&new_config.id, connection)? {
        return Err(ModifyNotificationConfigError::NotificationConfigAlreadyExists);
    }
    Ok(())
}

pub fn generate(
    DuplicateNotificationConfig { 
        id, //ID is already used for look up so we can assume it's the same
        title,
        kind,
        status,
        configuration_data,
        parameters,
        recipient_ids,
        recipient_list_ids,
        sql_recipient_list_ids,
    }: DuplicateNotificationConfig,
) -> Result<NotificationConfigRow, ModifyNotificationConfigError> {
    let mut new_configuration_data = "{}".to_string();
    if let Some(configuration_data) = configuration_data {
        new_configuration_data = configuration_data;
    }

    let mut new_parameters = "{}".to_string();
    if let Some(parameters) = parameters {
        new_parameters = parameters;
    }

    let mut new_recipient_ids = "[]".to_string(); 
    if let Some(recipient_ids) = recipient_ids {
        let recipient_json = serde_json::to_string(&recipient_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_recipient_ids = recipient_json;
    }

    let mut new_recipient_list_ids = "[]".to_string();
    if let Some(recipient_list_ids) = recipient_list_ids {
        let recipient_json = serde_json::to_string(&recipient_list_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_recipient_list_ids = recipient_json;
    }

    let mut new_sql_recipient_list_ids = "[]".to_string();
    if let Some(sql_recipient_list_ids) = sql_recipient_list_ids {
        let recipient_json = serde_json::to_string(&sql_recipient_list_ids).map_err(|_| {
            ModifyNotificationConfigError::BadUserInput(
                "Could not convert recipients to JSON".to_string(),
            )
        })?;
        new_sql_recipient_list_ids = recipient_json;
    }

    Ok(NotificationConfigRow {
        id,
        title: title.trim().to_string(),
        kind,
        configuration_data: new_configuration_data,
        status,
        parameters: new_parameters,
        recipient_ids: new_recipient_ids,
        recipient_list_ids: new_recipient_list_ids,
        sql_recipient_list_ids: new_sql_recipient_list_ids,
        last_run_datetime: None,
        next_due_datetime: None,
    })
}
