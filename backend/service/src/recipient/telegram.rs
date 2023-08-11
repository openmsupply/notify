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
            let cached_recipient = recipient_cache.entry(chat_id.clone()).or_insert_with(|| {
                match recipient_repo
                    .find_one_by_to_address_and_type(&chat_id, NotificationType::Telegram)
                {
                    Ok(Some(recepient)) => recepient,
                    Ok(None) => Default::default(), // Make sure default is created with unique id
                    Err(e) => {
                        log::error!("Error looking up recepient in database {}", e);
                        Default::default()
                    }
                }
            });

            // Check if we need to update the recipient name (e.g if the chat title has changed)
            if cached_recipient.name != chat.title {
                log::debug!(
                    "Chat title doesn't match recipient name, updating recipient: {:?}",
                    chat
                );
                cached_recipient.to_address = chat.id();
                cached_recipient.name = chat.title.clone();

                let upsert_recepient = UpsertRecepient {
                    id: cached_recipient.id.clone(),
                    name: chat.title.clone(),
                    notification_type: NotificationType::Telegram,
                    to_address: chat.id(),
                };

                match upsert_recepient(&ctx, upsert_recepient) {
                    Ok(recipient_result) => log::info!("Updated recipient: {:?}", recipient_result),
                    Err(e) => log::error!("Error updating recipient, skipping... {:?}", e),
                }
            }
        }
    }
}
