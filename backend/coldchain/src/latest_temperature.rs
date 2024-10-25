use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use diesel::dsl::sql;
use diesel::sql_types::Timestamp;
use diesel::RunQueryDsl;

table! {
    temperature_data (id) {
        id -> Text,
        temperature -> Nullable<Double>,
        date -> Date,
        time -> Timestamp,
        location_id -> Text,
        temperature_breach_id -> Text,
        store_id -> Text,
        sensor_id -> Text,
        log_interval -> Integer,
        om_datetime -> Text
    }
}

#[derive(Queryable, Debug, PartialEq, Clone)]
#[diesel(table_name = temperature_data)]
pub struct LatestTemperatureRow {
    #[diesel(sql_type = Text)]
    pub sensor_id: String,
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Timestamp)]
    pub log_datetime: chrono::NaiveDateTime,
    #[diesel(sql_type = Nullable<Double>)]
    pub temperature: Option<f64>,
}

pub fn latest_temperature(
    connection: &mut PgConnection,
    sensor_ids: Vec<String>,
) -> Result<Vec<LatestTemperatureRow>, DieselError> {
    let result = temperature_data::table
        .filter(temperature_data::sensor_id.eq_any(sensor_ids))
        .filter(temperature_data::temperature.lt(55.0)) // ignore any obviously bad data see: https://github.com/msupply-foundation/notify/issues/283
        .select((
            temperature_data::sensor_id,
            temperature_data::id,
            sql::<Timestamp>("CONCAT(TO_CHAR(date,'YYYY-MM-DD'),' ', TO_CHAR(time,'HH24:MI:SS'))::timestamp AS log_datetime"),
            temperature_data::temperature
        ))
        .order_by((
            temperature_data::sensor_id,
            temperature_data::date.desc(),
            temperature_data::time.desc()
        ))
        .distinct_on(temperature_data::sensor_id)
        .load(connection)?;

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
