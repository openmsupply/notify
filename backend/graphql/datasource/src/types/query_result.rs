use async_graphql::{Object, Union};
use graphql_core::simple_generic_errors::NodeError;
use service::datasource::QueryResult;

#[derive(Union)]
pub enum QueryResultResponse {
    Error(NodeError),
    Response(QueryResultNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct QueryResultNode {
    pub query_result: QueryResult,
}

#[Object]
impl QueryResultNode {
    pub async fn query(&self) -> &str {
        &self.row().query
    }

    pub async fn results(&self) -> &str {
        &self.row().results
    }

    pub async fn query_error(&self) -> &Option<std::string::String> {
        &self.row().query_error
    }
}

impl QueryResultNode {
    pub fn from_domain(query_result: QueryResult) -> QueryResultNode {
        QueryResultNode { query_result }
    }

    pub fn row(&self) -> &QueryResult {
        &self.query_result
    }
}
