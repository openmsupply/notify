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

#[cfg(test)]
mod test {
    #[test]
    fn test_check_recipient_exists() -> Result<(), String> {
        //
        // let result = check_recipient_exists().unwrap();
        // if result != expected {
        //     Err(format!(
        //         "check_username_doesnt_contain_special_characters {} result: {}, expected: {}",
        //         username, result, expected
        //     ))
        // } else {
        //     Ok(())
        // }

        Ok(())
    }
}
