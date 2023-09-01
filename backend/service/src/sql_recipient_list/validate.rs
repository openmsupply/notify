use repository::{
    EqualFilter, RepositoryError, SqlRecipientListFilter, SqlRecipientListRepository,
    SqlRecipientListRow, SqlRecipientListRowRepository, StorageConnection, StringFilter,
};
use util::is_valid_name;

pub fn check_list_name_doesnt_contain_special_characters(
    string: &str,
) -> Result<bool, RepositoryError> {
    Ok(!is_valid_name(string.trim()))
}

pub fn check_sql_recipient_list_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<SqlRecipientListRow>, RepositoryError> {
    SqlRecipientListRowRepository::new(connection).find_one_by_id(id)
}

pub fn check_sql_recipient_list_does_not_exist(
    id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let sql_recipient_list = check_sql_recipient_list_exists(id, connection)?;

    Ok(sql_recipient_list.is_none())
}

pub fn check_sql_recipient_list_name_is_unique(
    id: &str,
    list_name: Option<String>,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    match list_name {
        None => Ok(true),
        Some(list_name) => {
            let sql_recipient_lists = SqlRecipientListRepository::new(connection).query_by_filter(
                SqlRecipientListFilter::new()
                    .name(StringFilter::equal_to(&list_name.trim().to_string()))
                    .id(EqualFilter::not_equal_to(id)),
            )?;

            Ok(sql_recipient_lists.is_empty())
        }
    }
}

pub fn check_list_name_is_appropriate_length(name: &str) -> Result<bool, RepositoryError> {
    Ok(name.trim().len() >= 3 && name.len() <= 70)
}

#[cfg(test)]
mod test {
    use super::check_list_name_doesnt_contain_special_characters;

    fn list_name_char_test(name: &str, expected: bool) -> Result<(), String> {
        let result = check_list_name_doesnt_contain_special_characters(name).unwrap();
        if result != expected {
            Err(format!(
                "check_list_name_doesnt_contain_special_characters {} result: {}, expected: {}",
                name, result, expected
            ))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_good_names() -> Result<(), String> {
        [
            "admins",
            "Team A",
            "  Team 42 ",
            "Supervisors (Area 3)",
            "Friends: The close ones",
            "Monitors - fridges",
            "East & West regions",
        ]
        .iter()
        .try_for_each(|name| list_name_char_test(*name, true))?;

        Ok(())
    }

    #[test]
    fn test_bad_names() -> Result<(), String> {
        ["admi%", "Team A'); DROP TABLE Students;--"]
            .iter()
            .try_for_each(|name| list_name_char_test(*name, false))?;

        Ok(())
    }
}
