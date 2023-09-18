ALTER TABLE notification_config ADD COLUMN last_check_datetime TIMESTAMP;
ALTER TABLE notification_config ADD COLUMN next_check_datetime TIMESTAMP;