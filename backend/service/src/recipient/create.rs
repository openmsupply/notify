use super::{
    query::get_recipient,
    validate::{check_recipient_does_not_exist, check_to_address_is_unique},
    ModifyRecipientError,
};
use crate::audit_log::audit_log_entry;
use crate::service_provider::ServiceContext;

use chrono::Utc;
use repository::{
    LogType, NotificationType, Recipient, RecipientRow, RecipientRowRepository, StorageConnection,
};

#[derive(Clone)]
pub struct CreateRecipient {
    pub id: String,
    pub name: String,
    pub notification_type: NotificationType,
    pub to_address: String,
}

pub fn upsert_recipient(
    ctx: &ServiceContext,
    new_recipient: CreateRecipient,
) -> Result<Recipient, ModifyRecipientError> {
    let recipient = ctx
        .connection
        .transaction_sync(|connection| {
            let validation_result = validate(&new_recipient, connection);
            let new_recipient_row = match validation_result {
                Ok(_) => {
                    let new_recipient_row = generate(new_recipient.clone())?;
                    RecipientRowRepository::new(connection).insert_one(&new_recipient_row)?;
                    new_recipient_row
                }
                Err(ModifyRecipientError::RecipientAlreadyExists) => {
                    let new_recipient_row = generate(new_recipient.clone())?;
                    RecipientRowRepository::new(connection).update_one(&new_recipient_row)?;
                    new_recipient_row
                }
                Err(error) => {
                    return Err(error);
                }
            };

            get_recipient(ctx, new_recipient_row.id).map_err(ModifyRecipientError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::RecipientCreated,
        Some(new_recipient.id),
        Utc::now().naive_utc(),
    )?;

    Ok(recipient)
}

pub fn create_recipient(
    ctx: &ServiceContext,
    new_recipient: CreateRecipient,
) -> Result<Recipient, ModifyRecipientError> {
    let recipient = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_recipient, connection)?;
            let new_recipient_row = generate(new_recipient.clone())?;
            RecipientRowRepository::new(connection).insert_one(&new_recipient_row)?;

            get_recipient(ctx, new_recipient_row.id).map_err(ModifyRecipientError::from)
        })
        .map_err(|error| error.to_inner_error())?;

    // Audit logging
    audit_log_entry(
        &ctx,
        LogType::RecipientCreated,
        Some(new_recipient.id),
        Utc::now().naive_utc(),
    )?;

    Ok(recipient)
}

pub fn validate(
    new_recipient: &CreateRecipient,
    connection: &StorageConnection,
) -> Result<(), ModifyRecipientError> {
    if !check_recipient_does_not_exist(&new_recipient.id, connection)? {
        return Err(ModifyRecipientError::RecipientAlreadyExists);
    }

    if !check_to_address_is_unique(
        &new_recipient.id,
        Some(new_recipient.to_address.clone()),
        connection,
    )? {
        return Err(ModifyRecipientError::RecipientAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    CreateRecipient {
        id,
        name,
        notification_type,
        to_address,
    }: CreateRecipient,
) -> Result<RecipientRow, ModifyRecipientError> {
    Ok(RecipientRow {
        id,
        notification_type,
        name: name.trim().to_string(),
        to_address: to_address.trim().to_ascii_lowercase(),
    })
}
