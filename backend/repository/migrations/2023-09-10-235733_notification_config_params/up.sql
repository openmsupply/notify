ALTER TABLE notification_config ADD COLUMN parameters TEXT NOT NULL DEFAULT '[]';
ALTER TABLE sql_recipient_list RENAME COLUMN parameters TO required_parameters;