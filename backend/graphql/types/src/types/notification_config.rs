use super::{dataloader::DataLoader, LogNode};
use async_graphql::{Context, Enum, Object, SimpleObject, Union};
use graphql_core::{loader::AuditLogLoader, simple_generic_errors::NodeError, ContextExt};
use repository::{NotificationConfigKind, NotificationConfigStatus};
use serde::Serialize;
use service::{notification_config::query::NotificationConfig, ListResult};
use util::usize_to_u32;

#[derive(Union)]
pub enum NotificationConfigsResponse {
    Response(NotificationConfigConnector),
}

#[derive(Union)]
pub enum NotificationConfigResponse {
    Error(NodeError),
    Response(NotificationConfigNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct NotificationConfigNode {
    pub notification_config: NotificationConfig,
}

#[Object]
impl NotificationConfigNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }
    pub async fn title(&self) -> &str {
        &self.row().title
    }
    pub async fn kind(&self) -> ConfigKind {
        ConfigKind::from_domain(&self.row().kind)
    }
    pub async fn configuration_data(&self) -> &str {
        &self.row().configuration_data
    }
    pub async fn status(&self) -> ConfigStatus {
        ConfigStatus::from_domain(&self.row().status)
    }

    pub async fn parameters(&self) -> &str {
        &self.row().parameters
    }

    pub async fn recipient_ids(&self) -> &[String] {
        &self.row().recipient_ids
    }

    pub async fn recipient_list_ids(&self) -> &[String] {
        &self.row().recipient_list_ids
    }

    pub async fn sql_recipient_list_ids(&self) -> &[String] {
        &self.row().sql_recipient_list_ids
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

impl NotificationConfigNode {
    pub fn from_domain(notification_config: NotificationConfig) -> NotificationConfigNode {
        NotificationConfigNode {
            notification_config,
        }
    }

    pub fn row(&self) -> &NotificationConfig {
        &self.notification_config
    }
}

#[derive(SimpleObject)]
pub struct NotificationConfigConnector {
    total_count: u32,
    nodes: Vec<NotificationConfigNode>,
}

impl NotificationConfigConnector {
    pub fn from_domain(configs: ListResult<NotificationConfig>) -> NotificationConfigConnector {
        NotificationConfigConnector {
            total_count: configs.count,
            nodes: configs
                .rows
                .into_iter()
                .map(NotificationConfigNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(configs: Vec<NotificationConfig>) -> NotificationConfigConnector {
        NotificationConfigConnector {
            total_count: usize_to_u32(configs.len()),
            nodes: configs
                .into_iter()
                .map(NotificationConfigNode::from_domain)
                .collect(),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfigKind {
    ColdChain,
    Scheduled,
}

impl ConfigKind {
    pub fn to_domain(self) -> NotificationConfigKind {
        match self {
            ConfigKind::ColdChain => NotificationConfigKind::ColdChain,
            ConfigKind::Scheduled => NotificationConfigKind::Scheduled,
        }
    }

    pub fn from_domain(kind: &NotificationConfigKind) -> ConfigKind {
        match kind {
            NotificationConfigKind::ColdChain => ConfigKind::ColdChain,
            NotificationConfigKind::Scheduled => ConfigKind::Scheduled,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfigStatus {
    Disabled,
    Enabled,
}

impl ConfigStatus {
    pub fn to_domain(self) -> NotificationConfigStatus {
        match self {
            ConfigStatus::Enabled => NotificationConfigStatus::Enabled,
            ConfigStatus::Disabled => NotificationConfigStatus::Disabled,
        }
    }

    pub fn from_domain(status: &NotificationConfigStatus) -> ConfigStatus {
        match status {
            NotificationConfigStatus::Enabled => ConfigStatus::Enabled,
            NotificationConfigStatus::Disabled => ConfigStatus::Disabled,
        }
    }
}
