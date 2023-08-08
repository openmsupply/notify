# Notify Schema

Please look at the code to confirm schema as this might not be fully up to date.
It should provide an overview of the concepts though...

Conceptually, I'm thinking that notification groups could be created linking members from a query (e.g. all mSupply users with permission to a particular store)
We might implement it different ways, e.g. with an SQL query in the group itself, or maybe there's a process that refreshes the group members from a query on a schedule?
For MVP, we'll create of groups and members manually.

```mermaid
erDiagram
    NOTIFICATION_EVENT {
        TEXT id PK "UNIQUE NOT NULL"
        TEXT notification_group_id FK "NOT NULL"
        TEXT notification_type "NOT NULL (TELEGRAM/EMAIL/ETC)"
        TEXT to_address "NOT NULL (Email address/Chat_id/ETC)"
        TEXT status "NOT NULL"
        TIMESTAMP created_at "NOT NULL"
        TIMESTAMP updated_at "NOT NULL"
        TIMESTAMP send_at "NULLABLE"
        INTEGER retries "DEFAULT 0"
        TEXT message_content "NOT NULL (JSON?)"
        TEXT error_message "NULLABLE"
    }
    RECIPIENT_LIST {
        TEXT id PK "UNIQUE NOT NULL"
        TEXT name "NOT NULL"
        TEXT description "NOT NULL"
    }
    RECIPIENT_LIST_MEMBER {
        TEXT id PK "UNIQUE NOT NULL"
        TEXT recipient_list_id FK "NOT NULL"
        TEXT recipient_id FK "NOT NULL"
    }
    RECIPIENT {
        TEXT id PK "UNIQUE NOT NULL"
        TEXT name "NOT NULL"
        TEXT notification_type "NOT NULL (TELEGRAM/EMAIL/ETC)"
        TEXT to_address "NOT NULL (Email address/Chat_id/ETC)"
    }
    USER {
	    TEXT id PK "UNIQUE NOT NULL"
	    TEXT display_name "NOT NULL"
        TEXT username  "NOT NULL"
        TEXT hashed_password "NOT NULL (bcrypt)"
        TEXT email "NOT NULL"
    }
    USER_PERMISSION {
	    TEXT id PK "UNIQUE NOT NULL"
        TEXT user_id FK "NOT NULL"
        TEXT organisation_id FK ""
	    TEXT permission "NOT NULL"
    }
    RECIPIENT_LIST ||--o{ NOTIFICATION_EVENT : has
    RECIPIENT_LIST ||--o{ RECIPIENT_LIST_MEMBER : has
    RECIPIENT_LIST_MEMBER ||--o{ RECIPIENT : has
    USER ||--o{ USER_PERMISSION : has
```
