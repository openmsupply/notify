use super::validate::check_recipient_list_exists;
use crate::service_provider::ServiceContext;
use repository::{
    RecipientListMemberRowRepository, RecipientListRow, RecipientListRowRepository,
    RepositoryError, StorageConnection,
};

#[derive(PartialEq, Debug)]
pub enum DeleteRecipientListError {
    RecipientListDoesNotExist,
    DatabaseError(RepositoryError),
}

pub fn delete_recipient_list(
    ctx: &ServiceContext,
    recipient_list_id: &str,
) -> Result<String, DeleteRecipientListError> {
    let recipient_list = ctx
        .connection
        .transaction_sync(|connection| {
            let recipient_list_row = validate(connection, recipient_list_id)?;

            let member_repo = RecipientListMemberRowRepository::new(connection);
            let recipient_list_repo = RecipientListRowRepository::new(connection);

            match member_repo.delete_all_for_recipient_list_id(recipient_list_id) {
                Ok(_) => {
                    match recipient_list_repo.delete(recipient_list_id) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(DeleteRecipientListError::from(err));
                        }
                    };
                }
                Err(err) => {
                    return Err(DeleteRecipientListError::from(err));
                }
            };

            Ok(recipient_list_row)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(recipient_list.id)
}

pub fn validate(
    connection: &StorageConnection,
    recipient_list_id: &str,
) -> Result<RecipientListRow, DeleteRecipientListError> {
    let recipient_list_row = match check_recipient_list_exists(recipient_list_id, connection)? {
        Some(recipient_list_row) => recipient_list_row,
        None => return Err(DeleteRecipientListError::RecipientListDoesNotExist),
    };

    Ok(recipient_list_row)
}

impl From<RepositoryError> for DeleteRecipientListError {
    fn from(error: RepositoryError) -> Self {
        DeleteRecipientListError::DatabaseError(error)
    }
}
