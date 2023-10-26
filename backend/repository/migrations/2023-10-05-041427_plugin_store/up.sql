CREATE TABLE plugin_store (
    id TEXT NOT NULL PRIMARY KEY,
    plugin_name TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL -- Likely to be a JSON object
);

-- Unique index
CREATE UNIQUE INDEX plugin_store_plugin_name_key ON plugin_store (plugin_name, key);