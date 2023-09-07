CREATE TABLE
    sql_recipient_list (
        id TEXT NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        query TEXT NOT NULL,
        parameters TEXT NOT NULL, -- JSON array of strings ["region","user.name", "user.email"] Note: all params are assumed to be strings, you'll need to cast in your query if you need a float or int
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL
    );