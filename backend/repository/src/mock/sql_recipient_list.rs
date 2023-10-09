use crate::SqlRecipientListRow;

pub fn mock_sql_recipient_lists() -> Vec<SqlRecipientListRow> {
    vec![
        mock_sql_recipient_list_with_param(),
        mock_sql_recipient_list_with_no_param(),
    ]
}

pub fn mock_sql_recipient_list_with_param() -> SqlRecipientListRow {
    let sql_query = r#"SELECT '{{ email_address }}' as id, '{{ email_address }}' as name, 
                        'EMAIL' as notification_type, '{{ email_address }}' as to_address"#;
    SqlRecipientListRow {
        id: String::from("id_sql_recipient_list_with_param"),
        name: String::from("sql_recipient_list_with_param"),
        description: String::from("This is SQL Recipient List with an email address parameter, it should return a single row with to_address set to what ever you pass as the email_address parameter"),
        query: sql_query.to_string(),
        required_parameters: "[\"email_address\"]".to_string(),
        ..Default::default()
    }
}

pub fn mock_sql_recipient_list_with_no_param() -> SqlRecipientListRow {
    let sql_query = r#"SELECT 'id_no_param' as id, 'name_no_param' as name, 
                        'EMAIL' as notification_type, 'name_no_param@example.com' as to_address"#;
    SqlRecipientListRow {
        id: String::from("id_sql_recipient_list_no_param"),
        name: String::from("sql_recipient_list_no_param"),
        description: String::from("This is SQL Recipient List with no parameters, it should return a single row name_no_param@example.com as to_address"),
        query: sql_query.to_string(),
        required_parameters: "[]".to_string(),
        ..Default::default()
    }
}
