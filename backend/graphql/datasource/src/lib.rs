use async_graphql::*;
use graphql_core::standard_graphql_error::StandardGraphqlError::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::{
    auth::{Resource, ResourceAccessRequest},
    datasource::DatasourceServiceError,
};

// #[derive(Union)]
// pub enum SQLQueryResponse {
//     Response(String),
// }

// pub fn map_error(error: DatasourceServiceError) -> Result<SQLQueryResponse> {
//     let graphql_error = match error {
//         DatasourceServiceError::InternalError(e) => InternalError(e),
//         DatasourceServiceError::BadUserInput(e) => BadUserInput(e),
//         // Standard Graphql Errors
//     };

//     Err(graphql_error.extend())
// }

#[derive(Default, Clone)]
pub struct DatasourceQueries;

#[Object]
impl DatasourceQueries {
    pub async fn run_sql_query(&self, ctx: &Context<'_>, sql_query: String) -> Result<String> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_ctx = ctx.service_context(Some(&user))?;
        let datasource_service = &service_ctx.service_provider.datasource_service;
        // TODO some kind of query validation?

        // Query datasource service and return result
        let result = datasource_service.run_sql_query(sql_query).map_err(|e| {
            async_graphql::Error::new("Sorry not sure how to map these errors just yet!")
        })?;

        Ok(result)
    }
}
