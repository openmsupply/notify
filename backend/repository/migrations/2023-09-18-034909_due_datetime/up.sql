ALTER TABLE notification_config ADD COLUMN last_run_datetime TIMESTAMP;
ALTER TABLE notification_config ADD COLUMN next_due_datetime TIMESTAMP;