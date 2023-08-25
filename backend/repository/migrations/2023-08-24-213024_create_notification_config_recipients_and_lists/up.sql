CREATE table notification_config_recipient (
    id TEXT NOT NULL PRIMARY KEY,
    recipient_id TEXT NOT NULL REFERENCES recipient(id),
    notification_config_id TEXT NOT NULL REFERENCES notification_config(id)
);

CREATE table notification_config_recipient_list (
    id TEXT NOT NULL PRIMARY KEY,
    recipient_list_id TEXT NOT NULL REFERENCES recipient_list(id),
    notification_config_id TEXT NOT NULL REFERENCES notification_config(id)
);