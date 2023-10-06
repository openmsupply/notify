-- Your SQL goes here
ALTER TABLE notification_config ADD COLUMN recipient_ids TEXT NOT NULL default '[]';
ALTER TABLE notification_config ADD COLUMN recipient_list_ids TEXT NOT NULL default '[]';
ALTER TABLE notification_config ADD COLUMN sql_recipient_list_ids TEXT NOT NULL default '[]';