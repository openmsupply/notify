CREATE TABLE
    IF NOT EXISTS notification_event (
        id TEXT PRIMARY KEY,
        notification_config_id TEXT NULL, -- Null as we allow plugins to create notifications without a config definition
        notification_type TEXT NOT NULL,
        to_address TEXT NOT NULL, -- Telegram chatid, email address, something else in future?
        title TEXT NULL, -- May be null as some plugins might not need a title, mainly this is needed for email subjects, but could be used for other purposes
        message TEXT NOT NULL,
        status TEXT NOT NULL,
        sent_at TIMESTAMP NULL,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        retry_at TIMESTAMP NULL,
        error_message TEXT NULL
    );