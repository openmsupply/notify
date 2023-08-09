use super::validate::check_recipient_exists;
use crate::service_provider::ServiceContext;
use repository::{RecipientRow, RecipientRowRepository, RepositoryError, StorageConnection};

#[derive(PartialEq, Debug)]
pub enum DeleteRecipientError {
    RecipientDoesNotExist,
    DatabaseError(RepositoryError),
}

pub fn delete_recipient(
    ctx: &ServiceContext,
    recipient_id: &str,
) -> Result<String, DeleteRecipientError> {
    let recipient = ctx
        .connection
        .transaction_sync(|connection| {
            let recipient_row = validate(connection, recipient_id)?;

            let recipient_repo = RecipientRowRepository::new(connection);
            match recipient_repo.delete(recipient_id) {
                Ok(_) => {}
                Err(err) => {
                    return Err(DeleteRecipientError::from(err));
                }
            };
            Ok(recipient_row)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(recipient.id)
}

pub fn validate(
    connection: &StorageConnection,
    recipient_id: &str,
) -> Result<RecipientRow, DeleteRecipientError> {
    let recipient_row = match check_recipient_exists(recipient_id, connection)? {
        Some(recipient_row) => recipient_row,
        None => return Err(DeleteRecipientError::RecipientDoesNotExist),
    };

    Ok(recipient_row)
}

impl From<RepositoryError> for DeleteRecipientError {
    fn from(error: RepositoryError) -> Self {
        DeleteRecipientError::DatabaseError(error)
    }
}
