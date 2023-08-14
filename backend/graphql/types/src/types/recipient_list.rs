use super::{dataloader::DataLoader, LogNode, RecipientNode};
use async_graphql::{Context, Object, SimpleObject, Union};
use graphql_core::{
    loader::{AuditLogLoader, RecipientsLoader},
    simple_generic_errors::NodeError,
    ContextExt,
};
use repository::RecipientList;
use service::ListResult;
use util::usize_to_u32;

#[derive(Union)]
pub enum RecipientListsResponse {
    Response(RecipientListConnector),
}

#[derive(Union)]
pub enum RecipientListResponse {
    Error(NodeError),
    Response(RecipientListNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct RecipientListNode {
    pub recipient_list: RecipientList,
}

#[Object]
impl RecipientListNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }
    pub async fn name(&self) -> &str {
        &self.row().name
    }
    pub async fn description(&self) -> &str {
        &self.row().description
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

    pub async fn recipients(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<RecipientNode>, async_graphql::Error> {
        let loader = ctx.get_loader::<DataLoader<RecipientsLoader>>();
        let result = loader
            .load_one(self.row().id.to_string())
            .await?
            .unwrap_or_default();

        Ok(result.into_iter().map(RecipientNode::from_domain).collect())
    }
}

impl RecipientListNode {
    pub fn from_domain(recipient_list: RecipientList) -> RecipientListNode {
        RecipientListNode { recipient_list }
    }

    pub fn row(&self) -> &RecipientList {
        &self.recipient_list
    }
}

#[derive(SimpleObject)]
pub struct RecipientListConnector {
    total_count: u32,
    nodes: Vec<RecipientListNode>,
}

impl RecipientListConnector {
    pub fn from_domain(recipient_lists: ListResult<RecipientList>) -> RecipientListConnector {
        RecipientListConnector {
            total_count: recipient_lists.count,
            nodes: recipient_lists
                .rows
                .into_iter()
                .map(RecipientListNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(recipient_lists: Vec<RecipientList>) -> RecipientListConnector {
        RecipientListConnector {
            total_count: usize_to_u32(recipient_lists.len()),
            nodes: recipient_lists
                .into_iter()
                .map(RecipientListNode::from_domain)
                .collect(),
        }
    }
}
