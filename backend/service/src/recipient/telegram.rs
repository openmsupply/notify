use std::collections::HashMap;

use repository::{NotificationType, Recipient};
use repository::{RecipientRow, RecipientRowRepository};
use telegram::TelegramId;
use telegram::TelegramUpdate;
use util::uuid::uuid;

use crate::recipient::create::{upsert_recipient, CreateRecipient};
use crate::service_provider::ServiceContext;

fn blank_telegram_recipient() -> RecipientRow {
    RecipientRow {
        id: uuid(),
        name: "".to_string(),
        notification_type: NotificationType::Telegram,
        to_address: "".to_string(),
    }
}

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
                    Ok(Some(recipient)) => recipient,
                    Ok(None) => blank_telegram_recipient(),
                    Err(e) => {
                        log::error!("Error looking up recipient in database {}", e);
                        blank_telegram_recipient()
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

                let new_recipient = CreateRecipient {
                    id: cached_recipient.id.clone(),
                    name: chat.title.clone(),
                    notification_type: NotificationType::Telegram,
                    to_address: chat.id(),
                };

                match upsert_recipient(&ctx, new_recipient) {
                    Ok(recipient_result) => log::info!("Updated recipient: {:?}", recipient_result),
                    Err(e) => log::error!("Error updating recipient, skipping... {:?}", e),
                }
            }
        }
    }
}
