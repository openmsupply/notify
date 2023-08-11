use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn pg_sql_query_as_json(database_url: String) -> usize {
    PgConnection::establish(&database_url).unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_simple_select() {
        // EXAMPLE DATABASE_URL=postgres://postgres:postgres@localhost/postgres
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        assert_eq!(pg_sql_query_as_json(database_url), 1);

    }
}
