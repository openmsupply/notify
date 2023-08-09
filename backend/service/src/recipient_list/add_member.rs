use super::{
    validate::{check_recipient_list_exists, check_recipient_list_member_does_not_exist},
    ModifyRecipientListError,
};
use crate::{recipient::validate::check_recipient_exists, service_provider::ServiceContext};

use repository::{
    RecipientListMember, RecipientListMemberRow, RecipientListMemberRowRepository,
    StorageConnection,
};

#[derive(Clone)]
pub struct AddRecipientToList {
    pub id: String,
    pub recipient_id: String,
    pub recipient_list_id: String,
}
pub fn add_recipient_to_list(
    ctx: &ServiceContext,
    new_recipient_list_member: AddRecipientToList,
) -> Result<RecipientListMember, ModifyRecipientListError> {
    let recipient_list_member = ctx
        .connection
        .transaction_sync(|connection| {
            validate(&new_recipient_list_member, connection)?;
            let new_recipient_list_member_row = generate(new_recipient_list_member.clone())?;
            let repo = RecipientListMemberRowRepository::new(connection);

            repo.insert_one(&new_recipient_list_member_row)?;

            match repo
                .find_one_by_id(&new_recipient_list_member_row.id)
                .map_err(ModifyRecipientListError::from)?
            {
                Some(group_member) => Ok(group_member),
                None => Err(ModifyRecipientListError::RecipientListMemberDoesNotExist),
            }
            //
        })
        .map_err(|error| error.to_inner_error())?;

    // TODO: Audit logging // should this log go on the list or on the recipient? should really include both the list and recipient ids...
    // audit_log_entry(
    //     &ctx,
    //     LogType::RecipientAddedToList,
    //     Some(new_recipient_list.id),
    //     Utc::now().naive_utc(),
    // )?;

    Ok(recipient_list_member)
}

pub fn validate(
    new_member: &AddRecipientToList,
    connection: &StorageConnection,
) -> Result<(), ModifyRecipientListError> {
    match check_recipient_exists(&new_member.recipient_id, connection)? {
        Some(_) => (),
        None => return Err(ModifyRecipientListError::RecipientDoesNotExist),
    };

    match check_recipient_list_exists(&new_member.recipient_list_id, connection)? {
        Some(_) => (),
        None => return Err(ModifyRecipientListError::RecipientListDoesNotExist),
    };

    if !check_recipient_list_member_does_not_exist(
        &new_member.recipient_id,
        &new_member.recipient_list_id,
        connection,
    )? {
        return Err(ModifyRecipientListError::RecipientListMemberAlreadyExists);
    }

    Ok(())
}

pub fn generate(
    AddRecipientToList {
        id,
        recipient_id,
        recipient_list_id,
    }: AddRecipientToList,
) -> Result<RecipientListMemberRow, ModifyRecipientListError> {
    Ok(RecipientListMemberRow {
        id,
        recipient_id,
        recipient_list_id,
    })
}
