use async_graphql::{Object, SimpleObject, Union};
use graphql_core::simple_generic_errors::NodeError;

use repository::NotificationEvent;
use service::ListResult;
use util::usize_to_u32;

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
    pub async fn title(&self) -> String {
        self.row().title.to_owned().unwrap_or_default()
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
