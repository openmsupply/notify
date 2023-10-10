use super::{
    query::get_notification_query,
    validate::{
        check_list_name_doesnt_contain_special_characters, check_list_name_is_appropriate_length,
        check_notification_query_exists, check_notification_query_name_is_unique,
        check_notification_query_reference_name_is_unique,
    },
    ModifyNotificationQueryError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    LogType, NotificationQuery, NotificationQueryRow, NotificationQueryRowRepository,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct UpdateNotificationQuery {
    pub id: String,
    pub name: Option<String>,
    pub reference_name: Option<String>,
    pub description: Option<String>,
    pub query: Option<String>,
    pub required_parameters: Option<Vec<String>>,
}

pub fn update_notification_query(
    ctx: &ServiceContext,
    updated_notification_query: UpdateNotificationQuery,
) -> Result<NotificationQuery, ModifyNotificationQueryError> {
    let notification_query = ctx
        .connection
        .transaction_sync(|connection| {
            let notification_query_row = validate(connection, &updated_notification_query)?;
            let updated_notification_query_row =
                generate(updated_notification_query.clone(), notification_query_row)?;
            NotificationQueryRowRepository::new(connection)
                .update_one(&updated_notification_query_row)?;

            get_notification_query(ctx, updated_notification_query_row.id)
                .map_err(ModifyNotificationQueryError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::NotificationQueryUpdated,
        Some(updated_notification_query.id),
        Utc::now().naive_utc(),
    )?;
    Ok(notification_query)
}

pub fn validate(
    connection: &StorageConnection,
    new_notification_query: &UpdateNotificationQuery,
) -> Result<NotificationQueryRow, ModifyNotificationQueryError> {
    if let Some(list_name) = &new_notification_query.name {
        if !check_list_name_doesnt_contain_special_characters(list_name)? {
            return Err(ModifyNotificationQueryError::InvalidNotificationQueryName);
        }

        if !check_list_name_is_appropriate_length(&list_name)? {
            return Err(ModifyNotificationQueryError::InvalidNotificationQueryName);
        }
    }

    let notification_query_row =
        match check_notification_query_exists(&new_notification_query.id, connection)? {
            Some(notification_query_row) => notification_query_row,
            None => return Err(ModifyNotificationQueryError::NotificationQueryDoesNotExist),
        };

    if !check_notification_query_name_is_unique(
        &new_notification_query.id,
        new_notification_query.name.clone(),
        connection,
    )? {
        return Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists);
    }

    if !check_notification_query_reference_name_is_unique(
        &new_notification_query.id,
        new_notification_query.reference_name.clone(),
        connection,
    )? {
        return Err(ModifyNotificationQueryError::ReferenceNameAlreadyExists);
    }

    Ok(notification_query_row)
}

pub fn generate(
    UpdateNotificationQuery {
        id: _id, //ID is already used for look up so we can assume it's the same
        name,
        reference_name,
        description,
        query,
        required_parameters,
    }: UpdateNotificationQuery,
    current_notification_query_row: NotificationQueryRow,
) -> Result<NotificationQueryRow, ModifyNotificationQueryError> {
    let mut new_notification_query_row = current_notification_query_row;
    if let Some(name) = name {
        new_notification_query_row.name = name.trim().to_string();
    }
    if let Some(reference_name) = reference_name {
        new_notification_query_row.reference_name = reference_name.trim().to_string();
    }
    if let Some(description) = description {
        new_notification_query_row.description = description;
    }
    if let Some(query) = query {
        new_notification_query_row.query = query;
    }
    if let Some(parameters) = required_parameters {
        // Set parameters to JSON string
        let json_parameters = serde_json::to_value(parameters).map_err(|e| {
            ModifyNotificationQueryError::InternalError(format!(
                "Parameters can't be converted to JSON! - {:?}",
                e
            ))
        })?;
        let json_parameters = serde_json::to_string(&json_parameters).map_err(|e| {
            ModifyNotificationQueryError::InternalError(format!(
                "Parameters can't be converted to JSON string! - {:?}",
                e
            ))
        })?;

        new_notification_query_row.required_parameters = json_parameters;
    }

    Ok(new_notification_query_row)
}
