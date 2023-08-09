use repository::{
    EqualFilter, RecipientListMemberFilter, RecipientListMemberRepository, RecipientListMemberRow,
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
