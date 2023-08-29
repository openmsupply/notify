use crate::RecipientListRow;

pub fn mock_recipient_lists() -> Vec<RecipientListRow> {
    vec![
        mock_recipient_list_with_recipient_members_a_and_b(),
        mock_recipient_list_with_no_members(),
        mock_recipient_list_c(),
        mock_recipient_list_c2(),
        mock_recipient_list_all_msupply_emails(),
    ]
}

// recipient_list_members will need to be enabled
pub fn mock_recipient_list_with_recipient_members_a_and_b() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_a"),
        name: String::from("recipient_list_a"),
        description: String::from("This is Recipient List A"),
        sql_query: None,
    }
}

pub fn mock_recipient_list_with_no_members() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_b"),
        name: String::from("recipient_list_b"),
        description: String::from("This is Recipient List B"),
        sql_query: None,
    }
}

pub fn mock_recipient_list_c() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_c"),
        name: String::from("recipient_list_c"),
        description: String::from("This is Recipient List C"),
        sql_query: None,
    }
}

pub fn mock_recipient_list_c2() -> RecipientListRow {
    RecipientListRow {
        id: String::from("id_recipient_list_c2"),
        name: String::from("recipient_list_c2"),
        description: String::from("This is Recipient List C2"),
        sql_query: None,
    }
}

pub fn mock_recipient_list_all_msupply_emails() -> RecipientListRow {
    RecipientListRow { 
        id: String::from("id_all_mSupply_users"),
        name: "All mSupply Users".to_string(), 
        description: "Email addresses for all users in postgres database".to_string(), 
        sql_query: Some("SELECT id, name, 'EMAIL' as notification_type,e_mail as to_address  FROM \"user\" WHERE e_mail is not null and e_mail <> ''".to_string()) 
    }
}
