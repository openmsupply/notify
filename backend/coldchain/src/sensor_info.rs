use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use diesel::sql_types::{Double, Nullable, Text};
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug, PartialEq, Clone)]
#[diesel(table_name = sensor_info)]
pub struct SensorInfoRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub store_name: String,
    #[diesel(sql_type = Text)]
    pub store_id: String,
    #[diesel(sql_type = Text)]
    pub location_name: String,
    #[diesel(sql_type = Text)]
    pub sensor_name: String,
    #[diesel(sql_type = Nullable<Double>)]
    pub batterylevel: Option<f64>,
}

pub fn sensor_info(
    connection: &mut PgConnection,
    sensor_id: String,
) -> Result<Option<SensorInfoRow>, DieselError> {
    let query = "SELECT sn.id as id,
batterylevel, 
s.name as store_name, 
s.id as store_id,
coalesce(l.description, '') as location_name, 
sn.name as sensor_name 
FROM SENSOR sn 
JOIN store s ON sn.storeid = s.id 
LEFT JOIN location l on sn.locationid = l.id 
WHERE sn.id = $1 
    LIMIT 1";

    let query = sql_query(query).bind::<Text, _>(sensor_id);
    // println!("query: {:?}", query);
    let result: Option<SensorInfoRow> = query.get_result(connection).optional()?;
    Ok(result)
}

#[cfg(test)]
#[cfg(feature = "coldchain-tests")]
mod tests {
    use super::*;
    use std::env;

    /*
        These tests are more for development, to allow you to test the queries, it's not really designed to be run automatically, hence behind the coldchain-tests feature flag
        We'd need to setup a specific postgres db for the test cases, which could be done, but we'll see if it's worth it later...

         When we do https://github.com/openmsupply/notify/issues/176 this should change to use a test database
    */

    #[test]
    fn can_get_sensor_info() {
        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL environment variable must be set");

        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {} : {}", database_url, e));

        let sensor_id = "71dde2604abb11ed8c370d27b7187d58".to_string();
        let result = sensor_info(&mut connection, sensor_id).unwrap();
        println!("result: {:?}", result);
    }
}
