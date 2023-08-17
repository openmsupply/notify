# Notify Schema

Please look at the code to confirm schema as this might not be fully up to date.
This file should be considerd a guide only.

## Notification Events
Notification Events are used to record and track notifications sent by the system.
Eventually we'll build a UI to view these events and their status.
The system will also use this table to retry sending notifications that fail.
Each record will relate to a single notification sent to a single recipient.
> Note: Longer term we'll probably need to think about archiving old events.

> Note: Each notification event can be linked to a Notification Config. I've made this nullable just incase we want to send a notification that isn't linked to a config for some reason. Maybe even test notifications will be recorded here, which feels like a possible use case.

## Notification Config
Notification Config is used to store the "common" configuration for a notification. This includes things like a title, and who the messages should be sent to. Conceptually we want to allow different notication types to be managed via plugins. Based on the notication type, the "plugin" will be able to save all it's configuration in the `configuration_data` field. This will allow us to add new notification types without having to change the schema.
Some plugins might need or want to create and manage their own schema, but this should provide a good starting point.


```mermaid
erDiagram
    NOTIFICATION_CONFIG{
        TEXT id PK "UNIQUE NOT NULL"
        TEXT title "NOT NULL"
        TEXT type "NOT NULL (COLD_CHAIN/ETC)"
        TEXT recipient_ids "(JSON?)"
        TEXT recipient_list_ids "(JSON?)"
        TEXT configuration_data "NOT NULL (JSON)"

    }
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
    NOTIFICATION_CONFIG ||--o{ NOTIFICATION_EVENT : has
    RECIPIENT_LIST ||--o{ NOTIFICATION_EVENT : has
    RECIPIENT_LIST ||--o{ RECIPIENT_LIST_MEMBER : has
    RECIPIENT_LIST_MEMBER ||--o{ RECIPIENT : has
    USER ||--o{ USER_PERMISSION : has
```
