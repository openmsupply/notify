use super::{
    query::get_recipient_list, validate::check_recipient_list_does_not_exist,
    ModifyRecipientListError,
};
use crate::audit_log::audit_log_entry;
use crate::service_provider::ServiceContext;

use chrono::Utc;
use repository::{
    LogType, RecipientList, RecipientListRow, RecipientListRowRepository, StorageConnection,
};

#[derive(Clone)]
pub struct CreateRecipientList {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub fn create_recipient_list(
    ctx: &ServiceContext,
    new_recipient_list: CreateRecipientList,
) -> Result<RecipientList, ModifyRecipientListError> {
    let recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_recipient_list, connection)?;
            let new_recipient_list_row = generate(new_recipient_list.clone())?;
            RecipientListRowRepository::new(connection).insert_one(&new_recipient_list_row)?;

            get_recipient_list(ctx, new_recipient_list_row.id)
                .map_err(ModifyRecipientListError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::RecipientListCreated,
        Some(new_recipient_list.id),
        Utc::now().naive_utc(),
    )?;

    Ok(recipient_list)
}

pub fn validate(
    new_recipient_list: &CreateRecipientList,
    connection: &StorageConnection,
) -> Result<(), ModifyRecipientListError> {
    if !check_recipient_list_does_not_exist(&new_recipient_list.id, connection)? {
        return Err(ModifyRecipientListError::RecipientListAlreadyExists);
    }

    // TODO
    // list name has no special chars?
    // list name is also unique?
    // length constraints?

    Ok(())
}

pub fn generate(
    CreateRecipientList {
        id,
        name,
        description,
    }: CreateRecipientList,
) -> Result<RecipientListRow, ModifyRecipientListError> {
    Ok(RecipientListRow {
        id,
        name: name.trim().to_string(),
        description,
    })
}
