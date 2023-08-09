/*
   The telegram serviçe polls the telegram /getUpdates api and handle the following cases.

   A new chat_id is seen
       - Create a new recipient for the chat id
       - Send a welcome message to the chat (including the chat id for reference)

   An existing chat_id is seen (check if we need to update the chat name in the recipient)
       - No message is required

   A direct message is seen
       - Send the message with the chat id
*/

pub mod get_updates_poller;

/*
Example API Polling responses

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
