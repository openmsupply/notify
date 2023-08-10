use regex::Regex;
use repository::{
    EqualFilter, RecipientListFilter, RecipientListMemberFilter, RecipientListMemberRepository,
    RecipientListMemberRow, RecipientListRepository, RecipientListRow, RecipientListRowRepository,
    RepositoryError, StorageConnection, StringFilter,
};

lazy_static! {
    static ref SPECIAL_CHARS_RE: Regex = Regex::new(r"[^ 0-9A-Za-z_\-@.+:/]").unwrap();
}

pub fn check_list_name_doesnt_contain_special_characters(
    string: &str,
) -> Result<bool, RepositoryError> {
    Ok(!SPECIAL_CHARS_RE.is_match(string.trim()))
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
                    .name(StringFilter::equal_to(&list_name.trim().to_lowercase()))
                    .id(EqualFilter::not_equal_to(id)),
            )?;

            Ok(recipient_lists.is_empty())
        }
    }
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
