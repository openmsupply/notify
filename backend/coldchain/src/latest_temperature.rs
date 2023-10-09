use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use diesel::sql_types::{Double, Nullable, Text, Timestamp};
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug, PartialEq, Clone)]
#[diesel(table_name = temperature_data)]
pub struct LatestTemperatureRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub sensor_id: String,
    #[diesel(sql_type = Timestamp)]
    pub log_datetime: chrono::NaiveDateTime,
    #[diesel(sql_type = Nullable<Double>)]
    pub temperature: Option<f64>,
}

pub fn latest_temperature(
    connection: &mut PgConnection,
    sensor_id: String,
) -> Result<Option<LatestTemperatureRow>, DieselError> {
    let query = "SELECT 
    id,
    sensor_id,
    CONCAT(TO_CHAR(date,'YYYY-MM-DD'),' ', TO_CHAR(time,'HH24:MI:SS'))::timestamp AS log_datetime,
    temperature
    FROM temperature_log
    WHERE sensor_id = $1
    ORDER BY date DESC, time DESC
    LIMIT 1";

    let query = sql_query(query).bind::<Text, _>(sensor_id);
    // println!("query: {:?}", query);
    let result: Option<LatestTemperatureRow> = query.get_result(connection).optional()?;
    Ok(result)
}

#[cfg(test)]
#[cfg(feature = "coldchain-tests")]
mod tests {
    use super::*;
    use std::env;

    /*
        These tests are only useful for development at the moment to allow you to test the queries, it's not really designed to be run automatically, hence behind the coldchain-tests feature flag
        We'd need to setup a specific postgres db for the test cases, which could be done, but we'll see if it's worth it later...

        When we do https://github.com/openmsupply/notify/issues/176 this should change to use a test database
    */

    #[test]
    fn can_get_latest_temperatures() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");

        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        let sensor_id = "YOUR_SENSOR_ID_HERE".to_string();
        let result = latest_temperatures(&mut connection, sensor_id).unwrap();
        println!("result: {:?}", result);
    }
}
