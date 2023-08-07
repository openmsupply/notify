CREATE TABLE
    recipient (
        id TEXT NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        notification_type TEXT NOT NULL,
        to_address TEXT NOT NULL,
    );
