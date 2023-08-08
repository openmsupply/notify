use repository::{RecipientRow, RecipientRowRepository, RepositoryError, StorageConnection};

pub fn check_recipient_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<RecipientRow>, RepositoryError> {
    RecipientRowRepository::new(connection).find_one_by_id(id)
}

pub fn check_recipient_does_not_exist(
    id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let recipient = check_recipient_exists(id, connection)?;

    Ok(recipient.is_none())
}
