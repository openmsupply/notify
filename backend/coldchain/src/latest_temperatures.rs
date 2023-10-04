use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use diesel::sql_types::{Double, Nullable, Text, Timestamp};
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug, PartialEq)]
#[diesel(table_name = temperature_data)]
pub struct LatestTemperatureRow {
    #[diesel(sql_type = Text)]
    id: String,
    #[diesel(sql_type = Text)]
    pub sensor_id: String,
    #[diesel(sql_type = Timestamp)]
    pub log_datetime: chrono::NaiveDateTime,
    #[diesel(sql_type = Nullable<Double>)]
    pub temperature: Option<f64>,
}

pub fn latest_temperatures(
    connection: &mut PgConnection,
) -> Result<Vec<LatestTemperatureRow>, DieselError> {
    let query = "SELECT id, sensor_id, log_datetime, temperature FROM 
(SELECT 
    id,
    sensor_id,
    CONCAT(TO_CHAR(date,'YYYY-MM-DD'),' ', TO_CHAR(time,'HH24:MI:SS'))::timestamp AS log_datetime,
    temperature, 
    ROW_NUMBER() OVER (PARTITION BY sensor_id ORDER BY CONCAT(TO_CHAR(date,'YYYY-MM-DD'),' ', TO_CHAR(time,'HH24:MI:SS'))::timestamp DESC) rn										  
FROM temperature_log) as latest_temps 
WHERE rn=1
ORDER BY sensor_id";

    let results: Vec<LatestTemperatureRow> = sql_query(query).load(connection)?;

    Ok(results)
}

#[cfg(test)]
#[cfg(feature = "coldchain-tests")]
mod tests {
    use super::*;
    use std::env;

    /*
    These tests rely on a DATABASE_URL environment variable to be set
    e.g export DATABASE_URL=postgres://postgres:postgres@localhost/dashboard;
    or DATABASE_URL=postgres://postgres:postgres@localhost/postgres cargo test -- --nocapture

    The database need to to have temperature_log data in it.

    To use an environment variable from vs code set the URL in your settings
    "rust-analyzer.runnableEnv": {
        "DATABASE_URL": postgres://postgres:postgres@localhost/dashboard"
    }

    Finally if you want to disable these tests use
    `cargo test --no-default-features`
    */

    #[test]
    fn can_get_latest_temperatures() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");

        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        let result = latest_temperatures(&mut connection).unwrap();
        println!("result: {:?}", result);
    }
}
