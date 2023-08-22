use crate::{NotificationConfigKind, NotificationConfigRow};

pub fn mock_notification_configs() -> Vec<NotificationConfigRow> {
    vec![mock_notification_config_a(), mock_notification_config_b()]
}

pub fn mock_notification_config_a() -> NotificationConfigRow {
    NotificationConfigRow {
        id: String::from("id_notification_config_a"),
        title: String::from("Notification Config A"),
        kind: NotificationConfigKind::ColdChain,
        configuration_data: String::from("{ \"a\": \"some data\" }"),
    }
}

pub fn mock_notification_config_b() -> NotificationConfigRow {
    NotificationConfigRow {
        id: String::from("id_notification_config_b"),
        title: String::from("Notification Config B"),
        kind: NotificationConfigKind::ColdChain,
        configuration_data: String::from("{ \"b\": \"some data\" }"),
    }
}
