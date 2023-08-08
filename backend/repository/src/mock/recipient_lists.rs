use crate::RecipientListRow;

pub fn mock_recipient_lists() -> Vec<RecipientListRow> {
    vec![mock_recipient_list_a(), mock_recipient_list_b()]
}

pub fn mock_recipient_list_a() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_a"),
        name: String::from("recipient_list_a"),
        description: String::from("This is Recipient List A"),
    }
}

pub fn mock_recipient_list_b() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_b"),
        name: String::from("recipient_list_b"),
        description: String::from("This is Recipient List B"),
    }
}
