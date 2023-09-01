CREATE TABLE
    sql_recipient_list (
        id TEXT NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        query TEXT NOT NULL,
        parameters TEXT NOT NULL, -- JSON e.g. {"region":"string","tags":"string[]", "limit": "number"}
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL
    );
