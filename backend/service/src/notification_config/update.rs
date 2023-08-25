use super::{
    add_recipient::{add_recipient_to_notification_config, AddRecipientToNotificationConfig},
    query::get_notification_config,
    remove_recipient::{
        remove_recipient_from_notification_config, RemoveRecipientFromNotifcationConfig,
    },
    validate::check_notification_config_exists,
    ModifyNotificationConfigError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    EqualFilter, LogType, NotificationConfig, NotificationConfigRecipientFilter,
    NotificationConfigRecipientRepository, NotificationConfigRow, NotificationConfigRowRepository,
    Pagination, StorageConnection,
};

#[derive(Clone)]
pub struct UpdateNotificationConfig {
    pub id: String,
    pub title: Option<String>,
    pub configuration_data: Option<String>,
    pub recipient_ids: Option<Vec<String>>,
}

pub fn update_notification_config(
    ctx: &ServiceContext,
    updated_notification_config: UpdateNotificationConfig,
) -> Result<NotificationConfig, ModifyNotificationConfigError> {
    let notification_config = ctx
        .connection
        .transaction_sync(|connection| {
            let new_recipient_ids = updated_notification_config.recipient_ids.clone();

            let notification_config_row = validate(connection, &updated_notification_config)?;
            let updated_notification_config_row =
                generate(updated_notification_config.clone(), notification_config_row)?;

            NotificationConfigRowRepository::new(connection)
                .update_one(&updated_notification_config_row)?;

            if new_recipient_ids.is_some() {
                update_notification_config_recipients(
                    new_recipient_ids.unwrap(),
                    &updated_notification_config.id,
                    ctx,
                )?;
            }

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
        recipient_ids: _, // managed separately
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

    Ok(new_notification_config_row)
}

fn update_notification_config_recipients(
    new_recipient_ids: Vec<String>,
    notification_config_id: &str,
    ctx: &ServiceContext,
) -> Result<(), ModifyNotificationConfigError> {
    let existing_recipient_ids: Vec<String> =
        NotificationConfigRecipientRepository::new(&ctx.connection)
            .query(
                Pagination::all(),
                Some(
                    NotificationConfigRecipientFilter::new()
                        .notification_config_id(EqualFilter::equal_to(&notification_config_id)),
                ),
            )?
            .into_iter()
            .map(|config_recipient| config_recipient.recipient_id)
            .collect();

    let recipients_to_remove: Vec<String> = existing_recipient_ids
        .clone()
        .into_iter()
        .filter(|id| !new_recipient_ids.clone().contains(id))
        .collect();

    for id in recipients_to_remove {
        remove_recipient_from_notification_config(
            ctx,
            RemoveRecipientFromNotifcationConfig {
                notification_config_id: notification_config_id.to_string(),
                recipient_id: id,
            },
        )?;
    }

    let recipients_to_add: Vec<String> = new_recipient_ids
        .clone()
        .into_iter()
        .filter(|id| !existing_recipient_ids.clone().contains(id))
        .collect();

    for id in recipients_to_add {
        add_recipient_to_notification_config(
            ctx,
            AddRecipientToNotificationConfig {
                notification_config_id: notification_config_id.to_string(),
                recipient_id: id,
            },
        )?;
    }

    Ok(())
}
