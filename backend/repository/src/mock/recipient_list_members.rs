use crate::RecipientListMemberRow;

use super::{
    mock_recipient_a, mock_recipient_b, mock_recipient_list_with_recipient_members_a_and_b,
};

pub fn mock_recipient_list_members() -> Vec<RecipientListMemberRow> {
    vec![
        mock_recipient_list_member_a(),
        mock_recipient_list_member_b(),
    ]
}

pub fn mock_recipient_list_member_a() -> RecipientListMemberRow {
    RecipientListMemberRow {
        id: String::from("id_recipient_list_member_a"),
        recipient_id: mock_recipient_a().id.clone(),
        recipient_list_id: mock_recipient_list_with_recipient_members_a_and_b()
            .id
            .clone(),
    }
}

pub fn mock_recipient_list_member_b() -> RecipientListMemberRow {
    RecipientListMemberRow {
        id: String::from("id_recipient_list_member_b"),
        recipient_id: mock_recipient_b().id.clone(),
        recipient_list_id: mock_recipient_list_with_recipient_members_a_and_b()
            .id
            .clone(),
    }
}
