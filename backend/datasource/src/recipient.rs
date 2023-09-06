use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sql_types::*;
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug, PartialEq)]
#[diesel(table_name = basic_recipient)]
pub struct BasicRecipientRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub notification_type: String,
    #[diesel(sql_type = Text)]
    pub to_address: String,
}

pub fn pg_sql_query_as_recipients(
    connection: &mut PgConnection,
    sql_select_query: String,
) -> Result<Vec<BasicRecipientRow>, DieselError> {
    // Note: We may need to do some kind of validation of the SQL? We have to trust the uses to some degree though...

    let recipient_sql_query = format!(
        "WITH provided_query AS(
        {}
        ) SELECT id, name, notification_type, to_address FROM provided_query;",
        sql_select_query
    );

    let results: Vec<BasicRecipientRow> = sql_query(&recipient_sql_query).load(connection)?;

    Ok(results)
}

#[cfg(test)]
#[cfg(feature = "datasource-tests")]
mod tests {
    use super::*;
    use std::env;

    /*
    These tests rely on a DATABASE_URL environment variable to be set
    e.g export DATABASE_URL=postgres://postgres:postgres@localhost/postgres;
    or DATABASE_URL=postgres://postgres:postgres@localhost/postgres cargo test -- --nocapture

    To use an environment variable from vs code set the URL in your settings
    "rust-analyzer.runnableEnv": {
        "DATABASE_URL": postgres://postgres:postgres@localhost/postgres"
    }

    Finally if you want to disable these tests use
    `cargo test --no-default-features`
    */

    #[test]
    fn test_simple_select() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");

        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        let sql_query = r#"SELECT '1' as id, 'Name One' as name, 
                        'EMAIL' as notification_type, 'name1@example.com' as to_address"#;

        let result = pg_sql_query_as_recipients(&mut connection, sql_query.to_string()).unwrap();

        assert_eq!(
            result,
            vec![BasicRecipientRow {
                id: "1".to_string(),
                name: "Name One".to_string(),
                notification_type: "EMAIL".to_string(),
                to_address: "name1@example.com".to_string(),
            }]
        );
    }
}
