/*
   The telegram service polls the telegram /getUpdates api deserializes the messages (that is knows how to handle) and publishes them to a channel.

   We expect to implement logic like this:

   A new chat_id is seen
       - Create a new recipient for the chat id (via published update)
       - Send a welcome message to the chat (including the chat id for reference)

   An existing chat_id is seen (check if we need to update the chat name in the recipient)
       - No message is required

   A direct message is seen
       - Send the message with the chat id
*/

/*
Example API Polling responses
https://core.telegram.org/bots/api#getupdates

{
    "ok": true,
    "result": [
        {
            "update_id": 794348048,
            "message": {
                "message_id": 30,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691531796,
                "group_chat_created": true
            }
        },
        {
            "update_id": 794348049,
            "my_chat_member": {
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "date": 1691531796,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "member"
                }
            }
        },
        {
            "update_id": 794348050,
            "message": {
                "message_id": 31,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name (West)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691531864,
                "new_chat_title": "User1 & bot-name (West)"
            }
        },
        {
            "update_id": 794348051,
            "message": {
                "message_id": 32,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536017,
                "text": "@bot-name Can you tell me this chat_id please?",
                "entities": [
                    {
                        "offset": 0,
                        "length": 15,
                        "type": "mention"
                    }
                ]
            }
        },
        {
            "update_id": 794348052,
            "message": {
                "message_id": 33,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536034,
                "text": "This is a normal non direct message message..."
            }
        },
        {
            "update_id": 794348053,
            "message": {
                "message_id": 34,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -931768832,
                    "title": "Testing Notifications :)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536201,
                "text": "User2 Just wanting to check how mentions look that aren\u2019t too the bot\u2026",
                "entities": [
                    {
                        "offset": 0,
                        "length": 5,
                        "type": "text_mention",
                        "user": {
                            "id": 1320002934,
                            "is_bot": false,
                            "first_name": "User2",
                            "last_name": "Last2"
                        }
                    }
                ]
            }
        }
    ]
}
*/

use crate::{TelegramClient, TelegramUpdate};
use log;

static TELEGRAM_POLL_TIMEOUT_SECONDS: i64 = 30;
static ERROR_SLEEP_SECONDS: u64 = 10;

pub async fn poll_get_updates(
    telegram_client: &TelegramClient,
    tx_updates: &tokio::sync::mpsc::Sender<TelegramUpdate>,
) {
    // BTW : Might be a good idea to persist last_update_id to the KV store so we can pickup from where we left off on a restart?
    let mut last_update_id: i64 = -2;

    loop {
        let updates = telegram_client
            .get_updates(last_update_id, TELEGRAM_POLL_TIMEOUT_SECONDS)
            .await;
        match updates {
            Ok(updates) => {
                let num = updates.len();
                log::debug!("Got {} updates", num);
                if num > 0 {
                    last_update_id = handle_json_updates(updates, tx_updates).await;
                }
            }
            Err(error) => {
                log::error!("Error getting updates: {:?} \n Sleeping...", error);
                // Sleep for a bit so we don't hammer the CPU or telegram API
                tokio::time::sleep(std::time::Duration::from_secs(ERROR_SLEEP_SECONDS)).await;
            }
        };
    }
}

async fn handle_json_updates(
    updates: Vec<serde_json::Value>,
    tx_updates: &tokio::sync::mpsc::Sender<TelegramUpdate>,
) -> i64 {
    let mut last_update_id: i64 = -2;
    for update in updates {
        // First handle the update_id, if something goes wrong with parsing the JSON, we don't want to end up re-processing in an infinite loop
        let update_id = match update.get("update_id") {
            Some(update_id) => update_id,
            None => {
                log::error!(
                    "Error update doesn't include an update_id!!!!: {:?}",
                    update
                );
                // Increment update_id so we hopefully don't get this update again.
                last_update_id += 1;
                continue;
            }
        };
        let update_id = match update_id.as_i64() {
            Some(update_id) => update_id,
            None => {
                log::error!("Error parsing update_id as i64: {:?}", update);
                last_update_id += 1;
                continue;
            }
        };
        if update_id > last_update_id {
            last_update_id = update_id;
        }

        // Now try to parse the update using serde_json
        let telegram_update: TelegramUpdate = match serde_json::from_value(update.clone()) {
            Ok(telegram_update) => telegram_update,
            Err(error) => {
                log::error!("Error parsing update: {:?} update: {:?}", error, update);
                continue;
            }
        };

        // Send the update on the channel so other processors can handle it.
        let result = tx_updates.send(telegram_update).await;
        match result {
            Ok(_) => {
                log::debug!("Sent update to tx_updates");
            }
            Err(error) => {
                log::error!("Error sending message to tx_updates: {:?}", error);
            }
        };
    }
    last_update_id
}

#[cfg(test)]
mod test {
    use crate::{service::handle_json_updates, TelegramUpdate};

    #[tokio::test]
    async fn test_handle_json_updates_empty_vec() {
        let json = r#"[]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) = tokio::sync::mpsc::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == -2);
    }

    #[tokio::test]
    async fn test_handle_json_updates_single_message() {
        let json = r#"
        [
         {
            "update_id": 794348052,
            "message": {
                "message_id": 33,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536034,
                "text": "This is a normal non direct message message..."
            }
        }
        ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) = tokio::sync::mpsc::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == 794348052);
    }

    #[tokio::test]
    async fn test_handle_json_updates_single_my_chat_member_update() {
        let json = r#"
        [
        {
            "update_id": 794348049,
            "my_chat_member": {
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "date": 1691531796,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "member"
                }
            }
        }
        ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) = tokio::sync::mpsc::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == 794348049);
    }

    #[tokio::test]
    async fn test_handle_json_updates_lots_of_messages() {
        let json = r#"
       [
        {
            "update_id": 794348048,
            "message": {
                "message_id": 30,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691531796,
                "group_chat_created": true
            }
        },
        {
            "update_id": 794348049,
            "my_chat_member": {
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "date": 1691531796,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "bot-name",
                        "username": "bot-name"
                    },
                    "status": "member"
                }
            }
        },
        {
            "update_id": 794348050,
            "message": {
                "message_id": 31,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "User1 & bot-name (West)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691531864,
                "new_chat_title": "User1 & bot-name (West)"
            }
        },
        {
            "update_id": 794348051,
            "message": {
                "message_id": 32,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536017,
                "text": "@bot-name Can you tell me this chat_id please?",
                "entities": [
                    {
                        "offset": 0,
                        "length": 15,
                        "type": "mention"
                    }
                ]
            }
        },
        {
            "update_id": 794348052,
            "message": {
                "message_id": 33,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536034,
                "text": "This is a normal non direct message message..."
            }
        },
        {
            "update_id": 794348053,
            "message": {
                "message_id": 34,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -931768832,
                    "title": "Testing Notifications :)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536201,
                "text": "User2 Just wanting to check how mentions look that aren\u2019t too the bot\u2026",
                "entities": [
                    {
                        "offset": 0,
                        "length": 5,
                        "type": "text_mention",
                        "user": {
                            "id": 1320002934,
                            "is_bot": false,
                            "first_name": "User2",
                            "last_name": "Last2"
                        }
                    }
                ]
            }
        }
    ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) = tokio::sync::mpsc::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == 794348053);
    }

    #[tokio::test]
    async fn test_handle_json_updates_bad_data() {
        let json = r#"[{
            "update_id": 794348052,
            "message": {
                "message_id": 33,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "User1 & bot-name",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691536034,
                "text": "This is a normal non direct message message..."
            }
            },{
            "update_id": "Not a number",
            "not_a_message": {
                "not_a_message": 33,
                "date": 1691536034,
                "text": "This is a not a message!"
            }
        }]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) = tokio::sync::mpsc::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        // because we can't handle the second update_id as an i64, we automatically increment to avoid an infinite loop potential
        assert_eq!(last_update_id, 794348053);
    }
}
