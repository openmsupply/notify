use super::{
    query::get_sql_recipient_list,
    validate::{
        check_list_name_doesnt_contain_special_characters, check_list_name_is_appropriate_length,
        check_sql_recipient_list_does_not_exist, check_sql_recipient_list_name_is_unique,
    },
    ModifySqlRecipientListError,
};
use crate::audit_log::audit_log_entry;
use crate::service_provider::ServiceContext;

use chrono::Utc;
use repository::{
    LogType, SqlRecipientList, SqlRecipientListRow, SqlRecipientListRowRepository,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct CreateSqlRecipientList {
    pub id: String,
    pub name: String,
    pub description: String,
    pub query: String,
    pub required_parameters: Vec<String>,
}

pub fn create_sql_recipient_list(
    ctx: &ServiceContext,
    new_sql_recipient_list: CreateSqlRecipientList,
) -> Result<SqlRecipientList, ModifySqlRecipientListError> {
    let sql_recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_sql_recipient_list, connection)?;
            let new_sql_recipient_list_row = generate(new_sql_recipient_list.clone())?;
            SqlRecipientListRowRepository::new(connection)
                .insert_one(&new_sql_recipient_list_row)?;

            get_sql_recipient_list(ctx, new_sql_recipient_list_row.id)
                .map_err(ModifySqlRecipientListError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::SqlRecipientListCreated,
        Some(new_sql_recipient_list.id),
        Utc::now().naive_utc(),
    )?;

    Ok(sql_recipient_list)
}

pub fn validate(
    new_sql_recipient_list: &CreateSqlRecipientList,
    connection: &StorageConnection,
) -> Result<(), ModifySqlRecipientListError> {
    if !check_list_name_doesnt_contain_special_characters(&new_sql_recipient_list.name)? {
        return Err(ModifySqlRecipientListError::InvalidSqlRecipientListName);
    }

    if !check_list_name_is_appropriate_length(&new_sql_recipient_list.name)? {
        return Err(ModifySqlRecipientListError::InvalidSqlRecipientListName);
    }

    if !check_sql_recipient_list_does_not_exist(&new_sql_recipient_list.id, connection)? {
        return Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists);
    }

    if !check_sql_recipient_list_name_is_unique(
        &new_sql_recipient_list.id,
        Some(new_sql_recipient_list.name.clone()),
        connection,
    )? {
        return Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    CreateSqlRecipientList {
        id,
        name,
        description,
        query,
        required_parameters,
    }: CreateSqlRecipientList,
) -> Result<SqlRecipientListRow, ModifySqlRecipientListError> {
    let json_parameters = serde_json::to_value(required_parameters).map_err(|e| {
        ModifySqlRecipientListError::InternalError(format!(
            "Parameters can't be converted to JSON! - {:?}",
            e
        ))
    })?;
    let json_parameters = serde_json::to_string(&json_parameters).map_err(|e| {
        ModifySqlRecipientListError::InternalError(format!(
            "Parameters can't be converted to JSON string! - {:?}",
            e
        ))
    })?;

    Ok(SqlRecipientListRow {
        id,
        name: name.trim().to_string(),
        description,
        query,
        required_parameters: json_parameters,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    })
}
