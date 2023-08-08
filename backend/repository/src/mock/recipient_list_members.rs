use crate::RecipientListMemberRow;

pub fn mock_recipient_list_members() -> Vec<RecipientListMemberRow> {
    vec![
        mock_recipient_list_member_a(),
        mock_recipient_list_member_b(),
        mock_recipient_list_member_c(),
    ]
}

pub fn mock_recipient_list_member_a() -> RecipientListMemberRow {
    RecipientListMemberRow {
        id: String::from("id_recipient_list_member_a"),
        recipient_id: String::from("id_recipient_a"),
        recipient_list_id: String::from("id_recipient_list_a"),
    }
}

pub fn mock_recipient_list_member_b() -> RecipientListMemberRow {
    RecipientListMemberRow {
        id: String::from("id_recipient_list_member_b"),
        recipient_id: String::from("id_recipient_a"),
        recipient_list_id: String::from("id_recipient_list_b"),
    }
}

pub fn mock_recipient_list_member_c() -> RecipientListMemberRow {
    RecipientListMemberRow {
        id: String::from("id_recipient_list_member_c"),
        recipient_id: String::from("id_recipient_b"),
        recipient_list_id: String::from("id_recipient_list_a"),
    }
}
