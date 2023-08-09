use repository::{
    EqualFilter, RecipientFilter, RecipientRepository, RecipientRow, RecipientRowRepository,
    RepositoryError, StorageConnection, StringFilter,
};

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

pub fn check_to_address_is_unique(
    id: &str,
    to_address: Option<String>,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    match to_address {
        None => Ok(true),
        Some(to_address) => {
            let recipients = RecipientRepository::new(connection).query_by_filter(
                RecipientFilter::new()
                    .to_address(StringFilter::equal_to(&to_address.trim().to_lowercase()))
                    .id(EqualFilter::not_equal_to(id)),
            )?;

            Ok(recipients.is_empty())
        }
    }
}
