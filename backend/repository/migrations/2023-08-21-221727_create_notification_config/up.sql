CREATE TABLE
    notification_config (
        id TEXT NOT NULL PRIMARY KEY,
        title TEXT NOT NULL,
        kind TEXT NOT NULL,
        configuration_data TEXT NOT NULL
    );
