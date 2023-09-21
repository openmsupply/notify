use super::{
    query::get_notification_query,
    validate::{
        check_list_name_doesnt_contain_special_characters, check_list_name_is_appropriate_length,
        check_notification_query_does_not_exist, check_notification_query_name_is_unique,
    },
    ModifyNotificationQueryError,
};
use crate::audit_log::audit_log_entry;
use crate::service_provider::ServiceContext;

use chrono::Utc;
use repository::{
    LogType, NotificationQuery, NotificationQueryRow, NotificationQueryRowRepository,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct CreateNotificationQuery {
    pub id: String,
    pub name: String,
}

pub fn create_notification_query(
    ctx: &ServiceContext,
    new_notification_query: CreateNotificationQuery,
) -> Result<NotificationQuery, ModifyNotificationQueryError> {
    let notification_query = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_notification_query, connection)?;
            let new_notification_query_row = generate(new_notification_query.clone())?;
            NotificationQueryRowRepository::new(connection)
                .insert_one(&new_notification_query_row)?;

            get_notification_query(ctx, new_notification_query_row.id)
                .map_err(ModifyNotificationQueryError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::NotificationQueryCreated,
        Some(new_notification_query.id),
        Utc::now().naive_utc(),
    )?;

    Ok(notification_query)
}

pub fn validate(
    new_notification_query: &CreateNotificationQuery,
    connection: &StorageConnection,
) -> Result<(), ModifyNotificationQueryError> {
    if !check_list_name_doesnt_contain_special_characters(&new_notification_query.name)? {
        return Err(ModifyNotificationQueryError::InvalidNotificationQueryName);
    }

    if !check_list_name_is_appropriate_length(&new_notification_query.name)? {
        return Err(ModifyNotificationQueryError::InvalidNotificationQueryName);
    }

    if !check_notification_query_does_not_exist(&new_notification_query.id, connection)? {
        return Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists);
    }

    if !check_notification_query_name_is_unique(
        &new_notification_query.id,
        Some(new_notification_query.name.clone()),
        connection,
    )? {
        return Err(ModifyNotificationQueryError::NotificationQueryAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    CreateNotificationQuery { id, name }: CreateNotificationQuery,
) -> Result<NotificationQueryRow, ModifyNotificationQueryError> {
    Ok(NotificationQueryRow {
        id,
        name: name.trim().to_string(),
        description: "".to_string(),
        query: "".to_string(),
        required_parameters: "[]".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    })
}
