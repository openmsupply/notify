use super::{
    query::get_recipient_list,
    validate::{
        check_list_name_doesnt_contain_special_characters, check_list_name_is_appropriate_length,
        check_recipient_list_exists, check_recipient_list_name_is_unique,
    },
    ModifyRecipientListError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{
    LogType, RecipientList, RecipientListRow, RecipientListRowRepository, StorageConnection,
};

#[derive(Clone)]
pub struct UpdateRecipientList {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sql_query: Option<String>,
}

pub fn update_recipient_list(
    ctx: &ServiceContext,
    updated_recipient_list: UpdateRecipientList,
) -> Result<RecipientList, ModifyRecipientListError> {
    let recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            let recipient_list_row = validate(connection, &updated_recipient_list)?;
            let updated_recipient_list_row =
                generate(updated_recipient_list.clone(), recipient_list_row)?;
            RecipientListRowRepository::new(connection).update_one(&updated_recipient_list_row)?;

            get_recipient_list(ctx, updated_recipient_list_row.id)
                .map_err(ModifyRecipientListError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::RecipientListUpdated,
        Some(updated_recipient_list.id),
        Utc::now().naive_utc(),
    )?;
    Ok(recipient_list)
}

pub fn validate(
    connection: &StorageConnection,
    new_recipient_list: &UpdateRecipientList,
) -> Result<RecipientListRow, ModifyRecipientListError> {
    if let Some(list_name) = &new_recipient_list.name {
        if !check_list_name_doesnt_contain_special_characters(list_name)? {
            return Err(ModifyRecipientListError::InvalidRecipientListName);
        }

        if !check_list_name_is_appropriate_length(&list_name)? {
            return Err(ModifyRecipientListError::InvalidRecipientListName);
        }
    }

    let recipient_list_row = match check_recipient_list_exists(&new_recipient_list.id, connection)?
    {
        Some(recipient_list_row) => recipient_list_row,
        None => return Err(ModifyRecipientListError::RecipientListDoesNotExist),
    };

    if !check_recipient_list_name_is_unique(
        &new_recipient_list.id,
        new_recipient_list.name.clone(),
        connection,
    )? {
        return Err(ModifyRecipientListError::RecipientListAlreadyExists);
    }

    Ok(recipient_list_row)
}

pub fn generate(
    UpdateRecipientList {
        id: _id, //ID is already used for look up so we can assume it's the same
        name,
        description,
        sql_query,
    }: UpdateRecipientList,
    current_recipient_list_row: RecipientListRow,
) -> Result<RecipientListRow, ModifyRecipientListError> {
    let mut new_recipient_list_row = current_recipient_list_row;
    if let Some(name) = name {
        new_recipient_list_row.name = name.trim().to_string();
    }
    if let Some(description) = description {
        new_recipient_list_row.description = description;
    }
    if let Some(sql_query) = sql_query {
        new_recipient_list_row.sql_query = Some(sql_query);
    }

    Ok(new_recipient_list_row)
}
