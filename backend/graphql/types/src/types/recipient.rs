use super::{dataloader::DataLoader, LogNode};
use async_graphql::{Context, Enum, Object, SimpleObject, Union};
use graphql_core::{loader::AuditLogLoader, simple_generic_errors::NodeError, ContextExt};
use repository::{NotificationType, Recipient};
use serde::Serialize;
use service::{usize_to_u32, ListResult};

#[derive(Union)]
pub enum RecipientsResponse {
    Response(RecipientConnector),
}

#[derive(Union)]
pub enum RecipientResponse {
    Error(NodeError),
    Response(RecipientNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct RecipientNode {
    pub recipient: Recipient,
}

#[Object]
impl RecipientNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }
    pub async fn name(&self) -> &str {
        &self.row().name
    }
    pub async fn to_address(&self) -> &str {
        &self.row().to_address
    }
    pub async fn notification_type(&self) -> NotificationTypeNode {
        NotificationTypeNode::from_domain(&self.row().notification_type)
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

impl RecipientNode {
    pub fn from_domain(recipient: Recipient) -> RecipientNode {
        RecipientNode { recipient }
    }

    pub fn row(&self) -> &Recipient {
        &self.recipient
    }
}

#[derive(SimpleObject)]
pub struct RecipientConnector {
    total_count: u32,
    nodes: Vec<RecipientNode>,
}

impl RecipientConnector {
    pub fn from_domain(recipients: ListResult<Recipient>) -> RecipientConnector {
        RecipientConnector {
            total_count: recipients.count,
            nodes: recipients
                .rows
                .into_iter()
                .map(RecipientNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(recipients: Vec<Recipient>) -> RecipientConnector {
        RecipientConnector {
            total_count: usize_to_u32(recipients.len()),
            nodes: recipients
                .into_iter()
                .map(RecipientNode::from_domain)
                .collect(),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationTypeNode {
    Email,
    Telegram,
}

impl NotificationTypeNode {
    pub fn to_domain(self) -> NotificationType {
        match self {
            NotificationTypeNode::Email => NotificationType::Email,
            NotificationTypeNode::Telegram => NotificationType::Telegram,
        }
    }

    pub fn from_domain(notification_type: &NotificationType) -> NotificationTypeNode {
        match notification_type {
            NotificationType::Email => NotificationTypeNode::Email,
            NotificationType::Telegram => NotificationTypeNode::Telegram,
        }
    }
}
