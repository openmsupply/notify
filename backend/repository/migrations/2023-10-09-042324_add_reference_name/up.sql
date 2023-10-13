ALTER TABLE notification_query ADD COLUMN reference_name TEXT NOT NULL default id;
UPDATE notification_query SET reference_name = id;

CREATE UNIQUE INDEX ui_notification_query_reference_name ON notification_query(reference_name);