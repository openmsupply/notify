use crate::settings::Settings;
use datasource::{get_datasource_pool, pg_sql_query_as_json_rows, DatasourcePool};

// We use a trait for DatasourceService to allow mocking in tests
pub trait DatasourceServiceTrait: Send + Sync {
    fn run_sql_query(&self, sql_query: String) -> Result<String, DatasourceServiceError>;
}

pub struct DatasourceService {
    connection_pool: DatasourcePool,
}

#[derive(Debug)]
pub enum DatasourceServiceError {
    InternalError(String),
    BadUserInput(String),
}

impl DatasourceService {
    pub fn new(settings: Settings) -> Self {
        let connection_pool = get_datasource_pool(&settings.datasource);

        DatasourceService { connection_pool }
    }
}

impl DatasourceServiceTrait for DatasourceService {
    fn run_sql_query(&self, sql_query: String) -> Result<String, DatasourceServiceError> {
        let connection = &mut self.connection_pool.pool.get().map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not get connection from pool: {}",
                error
            ))
        })?;
        // Run query
        let result = pg_sql_query_as_json_rows(connection, sql_query).map_err(|error| {
            DatasourceServiceError::BadUserInput(format!("Could not run query: {}", error))
        })?;

        // TODO figure out what we acutally want!
        // For now let's put this results into a json value

        let json = serde_json::to_string(&result).map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not serialize query result: {}",
                error
            ))
        })?;

        Ok(json)
    }
}

#[cfg(test)]
#[cfg(feature = "datasource-tests")]
mod test {
    // TODO Test!
}
