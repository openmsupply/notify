use async_graphql::Enum;
use repository::NotificationEventStatus;
use serde::Serialize;

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventStatus {
    Queued,
    Sent,
    Errored, // Errored will be re-tried
    Failed,  // Failed will not be re-tried
}

impl EventStatus {
    pub fn to_domain(self) -> NotificationEventStatus {
        match self {
            EventStatus::Queued => NotificationEventStatus::Queued,
            EventStatus::Sent => NotificationEventStatus::Sent,
            EventStatus::Errored => NotificationEventStatus::Errored,
            EventStatus::Failed => NotificationEventStatus::Failed,
        }
    }

    pub fn from_domain(status: &NotificationEventStatus) -> EventStatus {
        match status {
            NotificationEventStatus::Queued => EventStatus::Queued,
            NotificationEventStatus::Sent => EventStatus::Sent,
            NotificationEventStatus::Errored => EventStatus::Errored,
            NotificationEventStatus::Failed => EventStatus::Failed,
        }
    }
}
