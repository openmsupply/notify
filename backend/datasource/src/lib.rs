use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sql_types::*;
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug, PartialEq)]
#[diesel(table_name = json_data)]
pub struct JsonDataRow {
    #[diesel(sql_type = Json)]
    data: serde_json::Value,
}

pub fn pg_sql_query_as_json(
    connection: &mut PgConnection,
    sql_select_query: String,
) -> Result<Vec<serde_json::Value>, DieselError> {
    // Note: We may need to do some kind of validation of the SQL? We have to trust the uses to some degree though...

    let json_row_sql_query = format!("WITH provided_query AS({}) SELECT row_to_json(provided_query) as data FROM provided_query;", sql_select_query);

    let json_results: Vec<JsonDataRow> = sql_query(&json_row_sql_query).load(connection)?;

    Ok(json_results.into_iter().map(|r| r.data).collect())
}

#[cfg(test)]
#[cfg(feature = "datasource-tests")]
mod tests {
    use super::*;
    use serde_json::json;
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

        let sql_query = r#"SELECT 1 as row_id, 'Row One' as description
                            UNION 
                            SELECT 2 as row_id, 'Row Two' as description"#;

        let result = pg_sql_query_as_json(&mut connection, sql_query.to_string()).unwrap();

        assert_eq!(
            result,
            vec![
                json!({"row_id": 1, "description": "Row One"}),
                json!({"row_id": 2, "description": "Row Two"})
            ]
        );
    }

    #[test]
    fn test_select_with_with() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");
        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        let sql_query = r#"WITH s1 as (SELECT 1 as row_id, 'Row One' as description), s2 as (SELECT 2 as row_id, 'Row Two' as description)
                           SELECT * from s1 UNION SELECT * from s2"#;

        let result = pg_sql_query_as_json(&mut connection, sql_query.to_string()).unwrap();

        assert_eq!(
            result,
            vec![
                json!({"row_id": 1, "description": "Row One"}),
                json!({"row_id": 2, "description": "Row Two"})
            ]
        );
    }

    #[test]
    fn test_invalid_query() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");
        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        // We probably don't want anyone running a query like this but still...
        let sql_query = r#"DROP TABLE users;"#;

        let result = pg_sql_query_as_json(&mut connection, sql_query.to_string());

        assert!(result.is_err());
    }
}
