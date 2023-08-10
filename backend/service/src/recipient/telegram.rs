use std::collections::HashMap;

use repository::RecipientRowRepository;
use repository::{NotificationType, Recipient};
use telegram::TelegramId;
use telegram::TelegramUpdate;
use util::uuid::uuid;

use crate::recipient::create::{self, create_recipient};
use crate::recipient::update::{update_recipient, UpdateRecipient};
use crate::service_provider::ServiceContext;

pub async fn handle_telegram_updates(
    ctx: ServiceContext,
    mut rx: tokio::sync::mpsc::Receiver<TelegramUpdate>,
) {
    // Technically should be an LRU cache but to avoid an additional dependency, we'll just use a HashMap.
    // We assume that the number of chats is small enough that we won't run out of memory ...
    // We need this cache to avoid hitting the sqlite database for every update.
    let mut chat_cache: HashMap<String, Recipient> = HashMap::new();
    let recipient_repo = RecipientRowRepository::new(&ctx.connection);

    while let Some(update) = rx.recv().await {
        log::info!("Received Telegram Update: {:?}", update);

        if let Some(chat) = update.chat() {
            let chat_id = chat.id();
            let cached_chat = chat_cache.get(&chat_id);
            if let Some(cached_chat) = cached_chat {
                if cached_chat.name != chat.title {
                    log::info!("New Chat Title, updating recipient: {:?}", chat);
                    let new_recipient = UpdateRecipient {
                        id: cached_chat.id.clone(),
                        name: Some(chat.title.clone()),
                        to_address: None,
                    };
                    let recipient_result = update_recipient(&ctx, new_recipient);
                    log::info!("Updated recipient: {:?}", recipient_result);
                    match recipient_result {
                        Ok(recipient) => {
                            chat_cache.insert(chat_id, recipient);
                        }
                        Err(_) => {
                            log::error!("Error creating recipient, skipping processing chat");
                            continue;
                        }
                    }
                }
            } else {
                log::info!("Found a chat we haven't seen before: {:?}", chat);
                let recipient_result = recipient_repo
                    .find_one_by_to_address_and_type(&chat_id, NotificationType::Telegram);
                match recipient_result {
                    Ok(recipient_option) => {
                        let recipient = match recipient_option {
                            Some(recipient) => {
                                log::debug!("Found recipient, updating: {:?}", recipient);

                                let new_recipient = UpdateRecipient {
                                    id: recipient.id,
                                    name: Some(chat.title.clone()),
                                    to_address: None,
                                };
                                let recipient_result = update_recipient(&ctx, new_recipient);
                                log::info!("Updated recipient: {:?}", recipient_result);
                                match recipient_result {
                                    Ok(recipient) => recipient,
                                    Err(_) => {
                                        log::error!(
                                            "Error creating recipient, skipping processing chat"
                                        );
                                        continue;
                                    }
                                }
                            }
                            None => {
                                log::info!("No recipient found, creating new one");
                                let new_recipient = create::CreateRecipient {
                                    id: uuid(),
                                    name: chat.title.clone(),
                                    notification_type: NotificationType::Telegram,
                                    to_address: chat_id.clone(),
                                };
                                let recipient_result = create_recipient(&ctx, new_recipient);
                                log::info!("Created recipient: {:?}", recipient_result);
                                match recipient_result {
                                    Ok(recipient) => recipient,
                                    Err(_) => {
                                        log::error!(
                                            "Error creating recipient, skipping processing chat"
                                        );
                                        continue;
                                    }
                                }
                            }
                        };
                        chat_cache.insert(chat_id, recipient);
                    }
                    Err(_) => {
                        log::error!("Error finding recipient from db, skipping processing chat");
                    }
                };
            }
        }
    }
}
