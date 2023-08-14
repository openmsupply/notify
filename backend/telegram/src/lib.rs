mod client;
pub mod service;

pub use client::*;
use serde::{Deserialize, Serialize};
// Rather than use an existing Telegram Client, we've implemented a minimal one in this crate.
// If we need more functionality we can flesh out this crate, or refactor using another library.
// We use serde to deserialize the json responses from telegram into structs with fields relevant to our application
//  - This comes with run-time risks if json can't be serialised. Hopefully the key edge cases are handled.

/*
"chat": {
    "id": -903279238,
    "title": "User1 & bot-name",
    "type": "group",
    "all_members_are_administrators": true
},
 */
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TelegramChat {
    pub id: i64,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub r#type: String,
}

impl TelegramChat {
    pub fn name(&self) -> String {
        let prefix = "Telegram:";

        match &self.title {
            Some(title) => title.clone(),
            None => {
                if let Some(first_name) = &self.first_name {
                    if let Some(last_name) = &self.last_name {
                        format!("{} {} {}", prefix, first_name, last_name)
                    } else {
                        first_name.clone()
                    }
                } else {
                    // If we don't have a title, first_name or last_name, we'll just use the id
                    format!("{} {}", prefix, self.id)
                }
            }
        }
    }
}

/*
   {
                  "id": 5068627745,
                  "is_bot": false,
                  "first_name": "User1",
                  "last_name": "Last1",
                  "language_code": "en"
   },
*/
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct TelegramUser {
    pub id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_bot: bool,
}

/*
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
*/

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramMessage {
    pub message_id: u64,
    pub text: Option<String>,
    pub from: TelegramUser,
    pub chat: TelegramChat,
}

/*
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
*/
// Note: TelegramMyChatMember is triggered when the bot is first added to a chat group
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramMyChatMember {
    pub chat: TelegramChat,
    pub from: TelegramUser,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum TelegramUpdateOrId {
    Update(TelegramUpdate),
    Id(TelegramUpdateWithId),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramUpdateWithId {
    pub update_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: Option<TelegramMessage>,
    pub my_chat_member: Option<TelegramMyChatMember>,
}

impl TelegramUpdate {
    // Returns the relevant chat for the update if available
    pub fn chat(&self) -> Option<TelegramChat> {
        if let Some(message) = &self.message {
            return Some(message.chat.clone());
        }
        if let Some(my_chat_member) = &self.my_chat_member {
            return Some(my_chat_member.chat.clone());
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramApiResponse {
    pub ok: bool,
    pub description: Option<String>,
    pub result: serde_json::Value,
    pub error_code: Option<i64>,
    pub retry_after: Option<i64>,
}

// Test cases for the TelegramUpdate struct
#[cfg(test)]
mod test {

    #[test]
    fn test_telegram_chat_name_update_message() {
        let json = r#"
        {
            "update_id": 794348060,
            "message": {
                "message_id": 43,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "Bob",
                    "last_name": "Jones",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "Bob & notify (East)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691620356,
                "new_chat_title": "Bob & notify (East)"
            }
        }
        "#;

        let update: super::TelegramUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(
            update.update_id,
            serde_json::Value::Number(794348060.into())
        );
        let message = update.message.unwrap();
        assert_eq!(message.message_id, 43);
        assert_eq!(message.text, None);
    }

    #[test]
    fn test_telegram_chat_update_message_with_text() {
        let json = r#"
        {
            "update_id": 794348060,
            "message": {
                "message_id": 43,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "Bob",
                    "last_name": "Jones",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "Bob & notify (East)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691620356,
                "new_chat_title": "Bob & notify (East)",
                "text": "Hello World"
            }
        }
        "#;

        let update: super::TelegramUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(
            update.update_id,
            serde_json::Value::Number(794348060.into())
        );
        let message = update.message.unwrap();
        assert_eq!(message.message_id, 43);
        assert_eq!(message.text, Some("Hello World".to_string()));
    }

    #[test]
    fn test_telegram_chat_update_my_chat_member() {
        let json = r#"
        {
            "update_id": 794348060,
            "my_chat_member": {
                "chat": {
                    "id": -903279238,
                    "title": "Bob & notify (East)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "Bob",
                    "last_name": "Jones",
                    "language_code": "en"
                },
                "date": 1691620356,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "notify",
                        "username": "notify"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "notify",
                        "username": "notify"
                    },
                    "status": "member"
                }
            }
        }
        "#;

        let update: super::TelegramUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.update_id, 794348060);
        let my_chat_member = update.my_chat_member.unwrap();
        assert_eq!(my_chat_member.chat.id, -903279238);
    }

    #[test]
    fn test_telegram_chat_update_message_direct_mention() {
        let json = r#"
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
        }
        "#;

        let update: super::TelegramUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(
            update.update_id,
            serde_json::Value::Number(794348051.into())
        );
        let message = update.message.unwrap();
        assert_eq!(message.message_id, 32);
        assert_eq!(
            message.text,
            Some("@bot-name Can you tell me this chat_id please?".to_string())
        );
        // TODO : Parse Entity stuff to detect direct mentions for https://github.com/openmsupply/notify/issues/32
    }

    #[test]
    fn test_get_chat_impl() {
        let json = r#"
        {
            "update_id": 794348060,
            "message": {
                "message_id": 43,
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "Bob",
                    "last_name": "Jones",
                    "language_code": "en"
                },
                "chat": {
                    "id": -903279238,
                    "title": "Bob & notify (East)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "date": 1691620356,
                "new_chat_title": "Bob & notify (East)"
            }
        }
        "#;

        let update: super::TelegramUpdate = serde_json::from_str(json).unwrap();
        let chat = update.chat().unwrap();
        assert_eq!(chat.id, -903279238);
        assert_eq!(chat.title, Some("Bob & notify (East)".to_string()));

        let json = r#"
        {
            "update_id": 794348060,
            "my_chat_member": {
                "chat": {
                    "id": -903279238,
                    "title": "Bob & notify (East)",
                    "type": "group",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 5068627745,
                    "is_bot": false,
                    "first_name": "Bob",
                    "last_name": "Jones",
                    "language_code": "en"
                },
                "date": 1691620356,
                "old_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "notify",
                        "username": "notify"
                    },
                    "status": "left"
                },
                "new_chat_member": {
                    "user": {
                        "id": 6544022299,
                        "is_bot": true,
                        "first_name": "notify",
                        "username": "notify"
                    },
                    "status": "member"
                }
            }
        }"#;

        let update = serde_json::from_str::<super::TelegramUpdate>(json).unwrap();
        let chat = update.chat().unwrap();
        assert_eq!(chat.id, -903279238);
        assert_eq!(chat.title, Some("Bob & notify (East)".to_string()));
    }
}
