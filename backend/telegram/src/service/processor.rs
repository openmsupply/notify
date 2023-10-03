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

use crate::{TelegramClient, TelegramUpdate, TelegramUpdateOrId};
use log::{self};

static TELEGRAM_POLL_TIMEOUT_SECONDS: i64 = 30;
static ERROR_SLEEP_SECONDS: u64 = 10;

pub async fn poll_get_updates(
    telegram_client: &TelegramClient,
    tx_updates: &tokio::sync::broadcast::Sender<TelegramUpdate>,
) {
    // BTW : Might be a good idea to persist last_update_id to the KV store so we can pickup from where we left off on a restart?
    let mut last_update_id = None;

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
    tx_updates: &tokio::sync::broadcast::Sender<TelegramUpdate>,
) -> Option<i64> {
    let mut last_update_id = None;
    for update in updates {
        // Now try to parse the update using serde_json
        let telegram_update: TelegramUpdateOrId = match serde_json::from_value(update.clone()) {
            Ok(telegram_update) => telegram_update,
            Err(error) => {
                log::error!("Error parsing update: {:?} update: {:?}", error, update);
                continue;
            }
        };

        let update_id = match telegram_update {
            TelegramUpdateOrId::Update(telegram_update) => {
                let update_id = telegram_update.update_id;
                // Send the update on the channel so other processors can handle it.
                let result = tx_updates.send(telegram_update);
                match result {
                    Ok(_) => {
                        log::debug!("Sent update to tx_updates");
                    }
                    Err(error) => {
                        log::error!("Error sending message to tx_updates: {:?}", error);
                    }
                };
                update_id
            }
            TelegramUpdateOrId::Id(id) => id.update_id,
        };

        // Update the last_update_id if it is bigger that the one we just saw
        last_update_id = match last_update_id {
            Some(id) if id > update_id => Some(id),
            _ => Some(update_id),
        };
    }
    last_update_id
}

#[cfg(test)]
mod test {
    use crate::{service::processor::handle_json_updates, TelegramUpdate};

    #[tokio::test]
    async fn test_handle_json_updates_empty_vec() {
        let json = r#"[]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == None);
    }

    #[tokio::test]
    async fn test_handle_json_updates_empty_message() {
        let json = r#"        [
         {
            "update_id": 1111,
            "something_else": {
                "date": 1691536034,
                "text": "We don't know what this is, but we do have a update_id"
            }
        }
        ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == Some(1111));
    }

    #[tokio::test]
    async fn test_handle_json_updates_invalid_message() {
        let json = r#"        [
         {
            "update_id": 1111,
             "message": {
                "message_id": 4444,
                "date": 1691536034,
                "text": "This is a direct message message..."
            }
        }
        ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert_eq!(last_update_id, Some(1111));
    }

    #[tokio::test]
    async fn test_handle_json_updates_single_private_message() {
        let json = r#"
        [
         {
            "update_id": 1111,
            "message": {
                "message_id": 4444,
                "from": {
                    "id": 5555,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": 5068627745,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "type": "private"
                },
                "date": 1691536034,
                "text": "This is a direct message message..."
            }
        }
        ]"#;
        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == Some(1111));

        // Get the message from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.message.is_some());
        let message = telegram_update.clone().message.unwrap();
        assert_eq!(message.message_id, 4444);
        assert_eq!(message.from.clone().unwrap_or_default().id, 5555);
        assert!(message.from.unwrap_or_default().is_bot == false);
        assert_eq!(
            telegram_update.chat().unwrap().name(),
            "Telegram: User1 Last1".to_string()
        );
        assert_eq!(message.text.unwrap(), "This is a direct message message...");
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
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == Some(794348052));

        // Get the message from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.message.is_some());
        let message = telegram_update.clone().message.unwrap();
        assert_eq!(message.message_id, 33);
        assert_eq!(message.from.clone().unwrap_or_default().id, 5068627745);
        assert!(message.from.unwrap_or_default().is_bot == false);
        assert_eq!(
            telegram_update.chat().unwrap().name(),
            "User1 & bot-name".to_string()
        );
        assert_eq!(
            message.text.unwrap(),
            "This is a normal non direct message message..."
        );
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
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == Some(794348049));
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
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert!(last_update_id == Some(794348053));
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
        let (tx, _rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        // We quietly skip the bad update_id
        assert_eq!(last_update_id, Some(794348052));
    }

    #[tokio::test]
    async fn test_handle_json_updates_added_to_group() {
        let json = r#"[
        {
            "update_id": 794348170,
            "my_chat_member": {
                "chat": {
                    "id": -914917543,
                    "title": "TestGroup1",
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
                "date": 1696280488,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "member"
                }
            }
        }
    ]"#;

        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert_eq!(last_update_id, Some(794348170));

        // Get the update from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.my_chat_member.is_some());
        let member = telegram_update.clone().my_chat_member.unwrap();
        assert_eq!(member.chat.id, -914917543);
        assert_eq!(member.new_chat_member.status, "member");
    }

    #[tokio::test]
    async fn test_handle_json_updates_added_to_channel() {
        let json = r#"[
        {
            "update_id": 794348166,
            "my_chat_member": {
                "chat": {
                    "id": -1001843666415,
                    "title": "Test Channel",
                    "type": "channel"
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "date": 1696280295,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "administrator",
                    "can_be_edited": false,
                    "can_manage_chat": true,
                    "can_change_info": false,
                    "can_post_messages": true,
                    "can_edit_messages": true,
                    "can_delete_messages": true,
                    "can_invite_users": false,
                    "can_restrict_members": true,
                    "can_promote_members": false,
                    "can_manage_video_chats": false,
                    "can_post_stories": false,
                    "can_edit_stories": false,
                    "can_delete_stories": false,
                    "is_anonymous": false,
                    "can_manage_voice_chats": false
                }
            }
        }
    ]"#;

        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert_eq!(last_update_id, Some(794348166));

        // Get the update from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.my_chat_member.is_some());
        let member = telegram_update.clone().my_chat_member.unwrap();
        assert_eq!(member.chat.id, -1001843666415);
        assert_eq!(member.new_chat_member.status, "administrator");
    }

    // Removed from a channel
    #[tokio::test]
    async fn test_handle_json_updates_removed_from_channel() {
        let json = r#"[
        {
            "update_id": 794348167,
            "my_chat_member": {
                "chat": {
                    "id": -1001843666415,
                    "title": "Test Channel",
                    "type": "channel"
                },
                "from": {
                    "id": 136817688,
                    "is_bot": true,
                    "first_name": "Channel",
                    "username": "Channel_Bot"
                },
                "date": 1696280385,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "administrator",
                    "can_be_edited": false,
                    "can_manage_chat": true,
                    "can_change_info": false,
                    "can_post_messages": true,
                    "can_edit_messages": true,
                    "can_delete_messages": true,
                    "can_invite_users": false,
                    "can_restrict_members": true,
                    "can_promote_members": false,
                    "can_manage_video_chats": false,
                    "can_post_stories": false,
                    "can_edit_stories": false,
                    "can_delete_stories": false,
                    "is_anonymous": false,
                    "can_manage_voice_chats": false
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "left"
                }
            }
        }
    ]"#;

        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert_eq!(last_update_id, Some(794348167));

        // Get the update from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.my_chat_member.is_some());
        let member = telegram_update.clone().my_chat_member.unwrap();
        assert_eq!(member.chat.id, -1001843666415);
        assert_eq!(member.new_chat_member.status, "left");
    }

    // Removed from group
    #[tokio::test]
    async fn test_handle_json_updates_removed_from_group() {
        let json = r#"[
        {
            "update_id": 794348168,
            "my_chat_member": {
                "chat": {
                    "id": -914917543,
                    "title": "Notify Bot Testing Group1",
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
                "date": 1696280442,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "member"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "botname",
                        "username": "botusername"
                    },
                    "status": "left"
                }
            }
        },
        {
            "update_id": 794348169,
            "message": {
                "message_id": 186,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "User1",
                    "last_name": "Last1",
                    "language_code": "en"
                },
                "chat": {
                    "id": -914917543,
                    "title": "Notify Bot Testing Group1",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1696280442,
                "left_chat_participant": {
                    "id": 6544022299,
                    "is_bot": true,
                    "first_name": "botname",
                    "username": "botusername"
                },
                "left_chat_member": {
                    "id": 6544022299,
                    "is_bot": true,
                    "first_name": "botname",
                    "username": "botusername"
                }
            }
        }
    ]"#;

        let updates: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        const TELEGRAM_UPDATE_BUFFER_SIZE: usize = 8;
        let (tx, mut rx) =
            tokio::sync::broadcast::channel::<TelegramUpdate>(TELEGRAM_UPDATE_BUFFER_SIZE);

        let last_update_id = handle_json_updates(updates, &tx).await;

        assert_eq!(last_update_id, Some(794348169));

        // Get the update from the channel
        let telegram_update = rx.recv().await.unwrap();
        assert!(telegram_update.my_chat_member.is_some());
        let member = telegram_update.clone().my_chat_member.unwrap();
        assert_eq!(member.chat.id, -914917543);
        assert_eq!(member.new_chat_member.status, "left");
    }
}
