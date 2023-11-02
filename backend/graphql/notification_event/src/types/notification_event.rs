use async_graphql::{Object, SimpleObject, Union};
use chrono::{DateTime, Utc};
use graphql_core::simple_generic_errors::NodeError;

use graphql_types::types::NotificationTypeNode;
use repository::NotificationEvent;
use service::ListResult;
use util::usize_to_u32;

use super::EventStatus;

#[derive(Union)]
pub enum NotificationEventsResponse {
    Response(NotificationEventConnector),
}

#[derive(Union)]
pub enum NotificationEventResponse {
    Error(NodeError),
    Response(NotificationEventNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct NotificationEventNode {
    pub notification_event: NotificationEvent,
}

#[Object]
impl NotificationEventNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }

    pub async fn notification_config_id(&self) -> Option<String> {
        self.row().notification_config_id.to_owned()
    }

    pub async fn title(&self) -> String {
        self.row().title.to_owned().unwrap_or_default()
    }
    pub async fn message(&self) -> &str {
        &self.row().message
    }
    pub async fn to_address(&self) -> &str {
        &self.row().to_address
    }
    pub async fn notification_type(&self) -> NotificationTypeNode {
        NotificationTypeNode::from_domain(&self.row().notification_type)
    }

    pub async fn error_message(&self) -> Option<String> {
        self.row().error_message.to_owned()
    }

    pub async fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.row().created_at, Utc)
    }
    pub async fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.row().updated_at, Utc)
    }
    pub async fn sent_at(&self) -> Option<DateTime<Utc>> {
        match self.row().sent_at {
            Some(sent_at) => Some(DateTime::<Utc>::from_utc(sent_at, Utc)),
            None => None,
        }
    }

    pub async fn status(&self) -> EventStatus {
        EventStatus::from_domain(&self.row().status)
    }
}

impl NotificationEventNode {
    pub fn from_domain(notification_event: NotificationEvent) -> NotificationEventNode {
        NotificationEventNode { notification_event }
    }

    pub fn row(&self) -> &NotificationEvent {
        &self.notification_event
    }
}

#[derive(SimpleObject)]
pub struct NotificationEventConnector {
    total_count: u32,
    nodes: Vec<NotificationEventNode>,
}

impl NotificationEventConnector {
    pub fn from_domain(
        notification_events: ListResult<NotificationEvent>,
    ) -> NotificationEventConnector {
        NotificationEventConnector {
            total_count: notification_events.count,
            nodes: notification_events
                .rows
                .into_iter()
                .map(NotificationEventNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(notification_events: Vec<NotificationEvent>) -> NotificationEventConnector {
        NotificationEventConnector {
            total_count: usize_to_u32(notification_events.len()),
            nodes: notification_events
                .into_iter()
                .map(NotificationEventNode::from_domain)
                .collect(),
        }
    }
}
