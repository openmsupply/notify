use async_graphql::{dataloader::DataLoader, Context, Object, SimpleObject, Union};
use graphql_core::{loader::AuditLogLoader, simple_generic_errors::NodeError, ContextExt};

use graphql_types::types::LogNode;
use repository::SqlRecipientList;
use service::ListResult;
use util::usize_to_u32;

#[derive(Union)]
pub enum SqlRecipientListsResponse {
    Response(SqlRecipientListConnector),
}

#[derive(Union)]
pub enum SqlRecipientListResponse {
    Error(NodeError),
    Response(SqlRecipientListNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct SqlRecipientListNode {
    pub recipient_list: SqlRecipientList,
}

#[Object]
impl SqlRecipientListNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }
    pub async fn name(&self) -> &str {
        &self.row().name
    }
    pub async fn description(&self) -> &str {
        &self.row().description
    }
    pub async fn query(&self) -> &str {
        &self.row().query
    }
    pub async fn parameters(&self) -> Result<Vec<String>, async_graphql::Error> {
        // Convert the parameters from a JSON array to a Vec<String>
        let parameters = serde_json::from_str::<Vec<String>>(&self.row().parameters)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(parameters)
    }

    pub async fn audit_logs(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LogNode>, async_graphql::Error> {
        let loader = ctx.get_loader::<DataLoader<AuditLogLoader>>();
        let result = loader
            .load_one(self.row().id.to_string())
            .await?
            .unwrap_or_default();

        Ok(result.into_iter().map(LogNode::from_domain).collect())
    }
}

impl SqlRecipientListNode {
    pub fn from_domain(recipient_list: SqlRecipientList) -> SqlRecipientListNode {
        SqlRecipientListNode { recipient_list }
    }

    pub fn row(&self) -> &SqlRecipientList {
        &self.recipient_list
    }
}

#[derive(SimpleObject)]
pub struct SqlRecipientListConnector {
    total_count: u32,
    nodes: Vec<SqlRecipientListNode>,
}

impl SqlRecipientListConnector {
    pub fn from_domain(recipient_lists: ListResult<SqlRecipientList>) -> SqlRecipientListConnector {
        SqlRecipientListConnector {
            total_count: recipient_lists.count,
            nodes: recipient_lists
                .rows
                .into_iter()
                .map(SqlRecipientListNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(recipient_lists: Vec<SqlRecipientList>) -> SqlRecipientListConnector {
        SqlRecipientListConnector {
            total_count: usize_to_u32(recipient_lists.len()),
            nodes: recipient_lists
                .into_iter()
                .map(SqlRecipientListNode::from_domain)
                .collect(),
        }
    }
}
