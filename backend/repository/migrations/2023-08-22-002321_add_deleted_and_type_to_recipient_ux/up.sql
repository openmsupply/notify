DROP INDEX IF EXISTS ux_recipient_to_address;
CREATE UNIQUE INDEX ux_recipient_to_address ON recipient (to_address, notification_type, deleted_datetime);