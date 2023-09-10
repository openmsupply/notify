use repository::{
    EqualFilter, RecipientListFilter, RecipientListMemberFilter, RecipientListMemberRepository,
    RecipientListMemberRow, RecipientListRepository, RecipientListRow, RecipientListRowRepository,
    RepositoryError, StorageConnection, StringFilter,
};
use util::is_valid_name;

pub fn check_list_name_doesnt_contain_special_characters(
    string: &str,
) -> Result<bool, RepositoryError> {
    Ok(!is_valid_name(string.trim()))
}

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

pub fn check_recipient_list_name_is_unique(
    id: &str,
    list_name: Option<String>,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    match list_name {
        None => Ok(true),
        Some(list_name) => {
            let recipient_lists = RecipientListRepository::new(connection).query_by_filter(
                RecipientListFilter::new()
                    .name(StringFilter::equal_to(&list_name.trim().to_string()))
                    .id(EqualFilter::not_equal_to(id)),
            )?;

            Ok(recipient_lists.is_empty())
        }
    }
}

pub fn check_list_name_is_appropriate_length(name: &str) -> Result<bool, RepositoryError> {
    Ok(name.trim().len() >= 3 && name.len() <= 70)
}

pub fn check_recipient_list_member_exists(
    recipient_id: &str,
    recipient_list_id: &str,
    connection: &StorageConnection,
) -> Result<Option<RecipientListMemberRow>, RepositoryError> {
    let filter = RecipientListMemberFilter::new()
        .recipient_id(EqualFilter::equal_to(recipient_id))
        .recipient_list_id(EqualFilter::equal_to(recipient_list_id));

    RecipientListMemberRepository::new(&connection).query_one(filter)
}

pub fn check_recipient_list_member_does_not_exist(
    recipient_id: &str,
    recipient_list_id: &str,
    connection: &StorageConnection,
) -> Result<bool, RepositoryError> {
    let list_member =
        check_recipient_list_member_exists(recipient_id, recipient_list_id, connection)?;

    Ok(list_member.is_none())
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
