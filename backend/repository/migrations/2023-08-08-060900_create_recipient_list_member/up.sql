CREATE table recipient_list_member (
    id TEXT NOT NULL PRIMARY KEY,
    recipient_list_id TEXT NOT NULL REFERENCES recipient_list(id),
    recipient_id TEXT NOT NULL REFERENCES recipient(id)
);