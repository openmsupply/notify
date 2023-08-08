use repository::{
    RecipientListRow, RecipientListRowRepository, RepositoryError, StorageConnection,
};

pub fn check_recipient_list_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<RecipientListRow>, RepositoryError> {
    RecipientListRowRepository::new(connection).find_one_by_id(id)
}

pub fn check_recipient_list_does_not_exist(
    id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let recipient_list = check_recipient_list_exists(id, connection)?;

    Ok(recipient_list.is_none())
}
