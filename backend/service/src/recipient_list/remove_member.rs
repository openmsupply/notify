use super::{validate::check_recipient_list_member_exists, ModifyRecipientListError};
use crate::{audit_log::audit_log_entry, service_provider::ServiceContext};

use chrono::Utc;
use repository::{
    LogType, RecipientListMember, RecipientListMemberRow, RecipientListMemberRowRepository,
    StorageConnection,
};

#[derive(Clone)]
pub struct RemoveRecipientFromList {
    pub recipient_id: String,
    pub recipient_list_id: String,
}
pub fn remove_recipient_from_list(
    ctx: &ServiceContext,
    remove_member: RemoveRecipientFromList,
) -> Result<RecipientListMember, ModifyRecipientListError> {
    let recipient_list_member = ctx
        .connection
        .transaction_sync(|connection| {
            let recipient_list_member = validate(&remove_member, connection)?;

            let member_row_repo = RecipientListMemberRowRepository::new(connection);

            match member_row_repo.delete(&recipient_list_member.id) {
                Ok(_) => {}
                Err(err) => return Err(ModifyRecipientListError::from(err)),
            }
            Ok(recipient_list_member)
        })
        .map_err(|error| error.to_inner_error())?;

    audit_log_entry(
        &ctx,
        LogType::RecipientRemovedFromList,
        Some(recipient_list_member.recipient_list_id.clone()),
        Utc::now().naive_utc(),
    )?;

    Ok(recipient_list_member)
}

pub fn validate(
    remove_member: &RemoveRecipientFromList,
    connection: &StorageConnection,
) -> Result<RecipientListMemberRow, ModifyRecipientListError> {
    let recipient_list_member = match check_recipient_list_member_exists(
        &remove_member.recipient_id,
        &remove_member.recipient_list_id,
        connection,
    )? {
        Some(recipient_list_member) => recipient_list_member,
        None => return Err(ModifyRecipientListError::RecipientListMemberDoesNotExist),
    };

    Ok(recipient_list_member)
}
