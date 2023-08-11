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
    // Technically we should be using an LRU cache but to avoid an additional dependency, we'll just use a HashMap.
    // We assume that the number of chats is small enough that we won't run out of memory ...
    // We need this cache to avoid hitting the sqlite database for every update.
    let mut recipient_cache: HashMap<String, Recipient> = HashMap::new();
    let recipient_repo = RecipientRowRepository::new(&ctx.connection);

    while let Some(update) = rx.recv().await {
        log::debug!("Received Telegram Update: {:?}", update);

        if let Some(chat) = update.chat() {
            let chat_id = chat.id();
            let cached_recipient = recipient_cache.get(&chat_id);
            let cached_recipient = match cached_recipient {
                Some(cached_recipient) => cached_recipient.to_owned(),
                None => {
                    // Cache miss, lets see if we already have it in the db, and if not create it
                    let recipient_result = recipient_repo
                        .find_one_by_to_address_and_type(&chat_id, NotificationType::Telegram);
                    match recipient_result {
                        Ok(recipient_option) => {
                            match recipient_option {
                                Some(recipient) => {
                                    // Recipient was already in the db, cache it and continue
                                    recipient_cache.insert(chat_id.clone(), recipient.clone());
                                    recipient
                                }
                                None => {
                                    log::debug!(
                                        "Recipient doesn't exist in db, creating new one..."
                                    );
                                    let new_recipient = create::CreateRecipient {
                                        id: uuid(),
                                        name: chat.title.clone(),
                                        notification_type: NotificationType::Telegram,
                                        to_address: chat_id.clone(),
                                    };
                                    let recipient_result = create_recipient(&ctx, new_recipient);
                                    log::info!("Created recipient: {:?}", recipient_result);
                                    match recipient_result {
                                        Ok(recipient) => {
                                            // Recipient created, cache it and continue
                                            recipient_cache
                                                .insert(chat_id.clone(), recipient.clone());
                                            recipient
                                        }
                                        Err(_) => {
                                            log::error!(
                                                "Error creating recipient, skipping processing chat"
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Error finding recipient in db, skipping... {:?}", e);
                            continue;
                        }
                    }
                }
            };

            // Check if we need to update the recipient name (e.g if the chat title has changed)
            if cached_recipient.name != chat.title {
                log::debug!(
                    "Chat title doesn't match recipient name, updating recipient: {:?}",
                    chat
                );
                let new_recipient = UpdateRecipient {
                    id: cached_recipient.id.clone(),
                    name: Some(chat.title.clone()),
                    to_address: None,
                };
                let recipient_result = update_recipient(&ctx, new_recipient);
                log::info!("Updated recipient: {:?}", recipient_result);
                match recipient_result {
                    Ok(recipient) => {
                        recipient_cache.insert(chat_id, recipient);
                    }
                    Err(e) => {
                        log::error!("Error updating recipient, skipping... {:?}", e);
                        continue;
                    }
                }
            }
        }
    }
}
