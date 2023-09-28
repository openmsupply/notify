-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notification_query (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT "",
    query TEXT NOT NULL DEFAULT "",
    required_parameters TEXT NOT NULL DEFAULT "[]", -- JSON array of strings ["region","user.name", "user.email"] Note: all params are assumed to be strings, you'll need to cast in your query if you need a float or int
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);