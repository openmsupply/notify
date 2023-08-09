use super::{
    query::get_recipient,
    validate::{check_recipient_exists, check_to_address_is_unique},
    ModifyRecipientError,
};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};
use chrono::Utc;
use repository::{LogType, Recipient, RecipientRow, RecipientRowRepository, StorageConnection};

#[derive(Clone)]
pub struct UpdateRecipient {
    pub id: String,
    pub name: Option<String>,
    pub to_address: Option<String>,
}

pub fn update_recipient(
    ctx: &ServiceContext,
    updated_recipient: UpdateRecipient,
) -> Result<Recipient, ModifyRecipientError> {
    let recipient = ctx
        .connection
        .transaction_sync(|connection| {
            let recipient_row = validate(connection, &updated_recipient)?;
            let updated_recipient_row = generate(updated_recipient.clone(), recipient_row)?;
            RecipientRowRepository::new(connection).update_one(&updated_recipient_row)?;

            get_recipient(ctx, updated_recipient_row.id).map_err(ModifyRecipientError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::RecipientUpdated,
        Some(updated_recipient.id),
        Utc::now().naive_utc(),
    )?;
    Ok(recipient)
}

pub fn validate(
    connection: &StorageConnection,
    new_recipient: &UpdateRecipient,
) -> Result<RecipientRow, ModifyRecipientError> {
    let recipient_row = match check_recipient_exists(&new_recipient.id, connection)? {
        Some(recipient_row) => recipient_row,
        None => return Err(ModifyRecipientError::RecipientDoesNotExist),
    };

    if !check_to_address_is_unique(
        &new_recipient.id,
        new_recipient.to_address.clone(),
        connection,
    )? {
        return Err(ModifyRecipientError::RecipientAlreadyExists);
    }

    Ok(recipient_row)
}

pub fn generate(
    UpdateRecipient {
        id: _id, //ID is already used for look up so we can assume it's the same
        name,
        to_address,
    }: UpdateRecipient,
    current_recipient_row: RecipientRow,
) -> Result<RecipientRow, ModifyRecipientError> {
    let mut new_recipient_row = current_recipient_row;
    if let Some(name) = name {
        new_recipient_row.name = name.trim().to_string();
    }
    if let Some(to_address) = to_address {
        new_recipient_row.to_address = to_address.trim().to_ascii_lowercase();
    }

    Ok(new_recipient_row)
}
