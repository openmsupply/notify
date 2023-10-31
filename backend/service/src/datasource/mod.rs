use crate::settings::Settings;
use datasource::{
    get_datasource_pool, pg_sql_query_as_json_rows, pg_sql_query_as_recipients, BasicRecipientRow,
    DatasourcePool,
};
use tera::{Context, Tera};

// We use a trait for DatasourceService to allow mocking in tests
pub trait DatasourceServiceTrait: Send + Sync {
    fn run_sql_query(&self, sql_query: String) -> Result<QueryResult, DatasourceServiceError>;
    fn run_sql_query_with_parameters(
        &self,
        sql_query: String,
        parameters: String,
    ) -> Result<QueryResult, DatasourceServiceError>;
    fn run_recipient_query(
        &self,
        sql_query: String,
    ) -> Result<Vec<BasicRecipientRow>, DatasourceServiceError>;
    fn get_connection_pool(&self) -> DatasourcePool;
}

pub struct DatasourceService {
    connection_pool: DatasourcePool,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct QueryResult {
    pub results: String,
    pub query: String,
    pub query_error: Option<String>,
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
    fn run_sql_query(&self, sql_query: String) -> Result<QueryResult, DatasourceServiceError> {
        let connection = &mut self.connection_pool.pool.get().map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not get connection from pool: {}",
                error
            ))
        })?;
        // Run query
        let result = pg_sql_query_as_json_rows(connection, sql_query.clone());
        let mut query_error = None;
        let result = match result{
            Ok(rows) => rows,
            Err(e) => {
                query_error = Some(format!("{:?}", e));
                vec![]// return empty array of results if there's an error
            }
        };

        // Serialize result as json
        let json = serde_json::to_string(&result).map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not serialize query result: {}",
                error
            ))
        })?;

        Ok(QueryResult {
            results: json,
            query: sql_query,
            query_error: query_error,
        })
    }
    fn run_recipient_query(
        &self,
        sql_query: String,
    ) -> Result<Vec<BasicRecipientRow>, DatasourceServiceError> {
        let connection = &mut self.connection_pool.pool.get().map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not get connection from pool: {}",
                error
            ))
        })?;
        // Run query
        let result = pg_sql_query_as_recipients(connection, sql_query).map_err(|error| {
            DatasourceServiceError::BadUserInput(format!("Could not run query: {}", error))
        })?;

        Ok(result)
    }

    fn run_sql_query_with_parameters(
        &self,
        sql_query: String,
        parameters: String,
    ) -> Result<QueryResult, DatasourceServiceError> {
        let connection = &mut self.connection_pool.pool.get().map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not get connection from pool: {}",
                error
            ))
        })?;

        // Parse Params as json
        let json_params = serde_json::from_str(&parameters).map_err(|e| {
            DatasourceServiceError::BadUserInput(format!(
                "Failed to parse params as json: {}",
                e.to_string()
            ))
        })?;

        // Pass params to template to get the full query
        let tera_context = Context::from_value(json_params).map_err(|e| {
            DatasourceServiceError::InternalError(format!(
                "Failed to convert params to tera context: {}",
                e.to_string()
            ))
        })?;

        let full_query = Tera::one_off(&sql_query, &tera_context, false).map_err(|e| {
            DatasourceServiceError::InternalError(format!(
                "Failed to parse query as tera template: {}",
                e.to_string()
            ))
        })?;

        // Run query
        let result = pg_sql_query_as_json_rows(connection, full_query.clone());
        let mut query_error = None;
        let result = match result{
            Ok(rows) => rows,
            Err(e) => {
                query_error = Some(format!("{:?}", e));
                vec![]// return empty array of results if there's an error
            }
        };

        // Serialize result as json
        let json = serde_json::to_string(&result).map_err(|error| {
            DatasourceServiceError::InternalError(format!(
                "Could not serialize query result: {}",
                error
            ))
        })?;

        Ok(QueryResult {
            results: json,
            query: full_query,
            query_error: query_error,
        })
    }

    fn get_connection_pool(&self) -> DatasourcePool {
        self.connection_pool.clone()
    }
}

#[cfg(test)]
#[cfg(feature = "datasource-tests")]
mod test {}
