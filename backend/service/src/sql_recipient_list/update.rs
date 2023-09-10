use super::{
    query::get_sql_recipient_list,
    validate::{
        check_list_name_doesnt_contain_special_characters, check_list_name_is_appropriate_length,
        check_sql_recipient_list_exists, check_sql_recipient_list_name_is_unique,
    },
    ModifySqlRecipientListError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    LogType, SqlRecipientList, SqlRecipientListRow, SqlRecipientListRowRepository,
    StorageConnection,
};

#[derive(Clone, Default)]
pub struct UpdateSqlRecipientList {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub query: Option<String>,
    pub parameters: Option<Vec<String>>,
}

pub fn update_sql_recipient_list(
    ctx: &ServiceContext,
    updated_sql_recipient_list: UpdateSqlRecipientList,
) -> Result<SqlRecipientList, ModifySqlRecipientListError> {
    let sql_recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            let sql_recipient_list_row = validate(connection, &updated_sql_recipient_list)?;
            let updated_sql_recipient_list_row =
                generate(updated_sql_recipient_list.clone(), sql_recipient_list_row)?;
            SqlRecipientListRowRepository::new(connection)
                .update_one(&updated_sql_recipient_list_row)?;

            get_sql_recipient_list(ctx, updated_sql_recipient_list_row.id)
                .map_err(ModifySqlRecipientListError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::SqlRecipientListUpdated,
        Some(updated_sql_recipient_list.id),
        Utc::now().naive_utc(),
    )?;
    Ok(sql_recipient_list)
}

pub fn validate(
    connection: &StorageConnection,
    new_sql_recipient_list: &UpdateSqlRecipientList,
) -> Result<SqlRecipientListRow, ModifySqlRecipientListError> {
    if let Some(list_name) = &new_sql_recipient_list.name {
        if !check_list_name_doesnt_contain_special_characters(list_name)? {
            return Err(ModifySqlRecipientListError::InvalidSqlRecipientListName);
        }

        if !check_list_name_is_appropriate_length(&list_name)? {
            return Err(ModifySqlRecipientListError::InvalidSqlRecipientListName);
        }
    }

    let sql_recipient_list_row =
        match check_sql_recipient_list_exists(&new_sql_recipient_list.id, connection)? {
            Some(sql_recipient_list_row) => sql_recipient_list_row,
            None => return Err(ModifySqlRecipientListError::SqlRecipientListDoesNotExist),
        };

    if !check_sql_recipient_list_name_is_unique(
        &new_sql_recipient_list.id,
        new_sql_recipient_list.name.clone(),
        connection,
    )? {
        return Err(ModifySqlRecipientListError::SqlRecipientListAlreadyExists);
    }

    Ok(sql_recipient_list_row)
}

pub fn generate(
    UpdateSqlRecipientList {
        id: _id, //ID is already used for look up so we can assume it's the same
        name,
        description,
        query,
        parameters,
    }: UpdateSqlRecipientList,
    current_sql_recipient_list_row: SqlRecipientListRow,
) -> Result<SqlRecipientListRow, ModifySqlRecipientListError> {
    let mut new_sql_recipient_list_row = current_sql_recipient_list_row;
    if let Some(name) = name {
        new_sql_recipient_list_row.name = name.trim().to_string();
    }
    if let Some(description) = description {
        new_sql_recipient_list_row.description = description;
    }
    if let Some(query) = query {
        new_sql_recipient_list_row.query = query;
    }
    if let Some(parameters) = parameters {
        // Set parameters to JSON string
        let json_parameters = serde_json::to_value(parameters).map_err(|e| {
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

        new_sql_recipient_list_row.parameters = json_parameters;
    }

    Ok(new_sql_recipient_list_row)
}
