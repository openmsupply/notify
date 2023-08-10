mod client;
pub mod service;

pub use client::*;
use serde::{Deserialize, Serialize};

// Rather than use a defined Telegram Client, we've implemented a minimal one in this crate.
// The expectation is that as we need more functionality we can flesh out this infrastructure
// We use serde to deserialize the json responses from telegram into structs with fields relevant to our application

// Get a telegram id from a struct as a String
// This should be a derive macro eventually, but one thing at at time!
pub trait TelegramId {
    fn id(&self) -> String;
}

/*
"chat": {
    "id": -903279238,
    "title": "User1 & bot-name",
    "type": "group",
    "all_members_are_administrators": true
},
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramChat {
    pub id: serde_json::Value,
    pub title: String,
    pub r#type: String,
}

impl TelegramId for TelegramChat {
    fn id(&self) -> String {
        self.id.to_string()
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
#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramUser {
    pub id: serde_json::Value,
    pub username: Option<String>,
    pub is_bot: bool,
}

impl TelegramId for TelegramUser {
    fn id(&self) -> String {
        self.id.to_string()
    }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramMessage {
    pub message_id: serde_json::Value,
    pub text: Option<String>,
    pub from: TelegramUser,
    pub chat: TelegramChat,
}
impl TelegramId for TelegramMessage {
    fn id(&self) -> String {
        self.message_id.to_string()
    }
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
// TelegramMyChatMember is used to quickly add chatids when the bot is first added to a chat group
#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramMyChatMember {
    pub chat: TelegramChat,
    pub from: TelegramUser,
    pub date: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramUpdate {
    pub update_id: serde_json::Value,
    pub message: Option<TelegramMessage>,
    pub my_chat_member: Option<TelegramMyChatMember>,
}
impl TelegramId for TelegramUpdate {
    fn id(&self) -> String {
        self.update_id.to_string()
    }
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
        assert_eq!(
            update.update_id,
            serde_json::Value::Number(794348060.into())
        );
        let my_chat_member = update.my_chat_member.unwrap();
        assert_eq!(
            my_chat_member.chat.id,
            serde_json::Value::Number((-903279238).into())
        );
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
        // TODO : Parse Entity stuff
    }
    // TODO : Test cases for TelegramUpdate impl (eg. chat())
}
