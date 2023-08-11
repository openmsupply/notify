use crate::RecipientListRow;

pub fn mock_recipient_lists() -> Vec<RecipientListRow> {
    vec![
        mock_recipient_list_with_recipient_members_a_and_b(),
        mock_recipient_list_with_no_members(),
        mock_recipient_list_c(),
    ]
}

// recipient_list_members will need to be enabled
pub fn mock_recipient_list_with_recipient_members_a_and_b() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_a"),
        name: String::from("recipient_list_a"),
        description: String::from("This is Recipient List A"),
    }
}

pub fn mock_recipient_list_with_no_members() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_b"),
        name: String::from("recipient_list_b"),
        description: String::from("This is Recipient List B"),
    }
}

pub fn mock_recipient_list_c() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_c"),
        name: String::from("recipient_list_c"),
        description: String::from("This is Recipient List C"),
    }
}
