use std::collections::HashMap;

use repository::{NotificationType, Recipient, RecipientRow, RecipientRowRepository};
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
        deleted_datetime: None,
    }
}

pub async fn handle_telegram_updates(
    ctx: ServiceContext,
    channel: &tokio::sync::broadcast::Sender<TelegramUpdate>,
) {
    // Technically we should be using an LRU cache but to avoid an additional dependency, we'll just use a HashMap.
    // We assume that the number of chats is small enough that we won't run out of memory ...
    // We need this cache to avoid hitting the sqlite database for every update.
    let mut recipient_cache: HashMap<String, Recipient> = HashMap::new();
    let recipient_repo = RecipientRowRepository::new(&ctx.connection);

    let mut rx = channel.subscribe();

    loop {
        let update = match rx.recv().await {
            Ok(update) => {
                log::debug!(
                    "Received Telegram Update Recipient Ready to respond: {:?}",
                    update
                );
                update
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                log::error!("Telegram update lagged behind, something is delaying processing off telegram recipient events!");
                continue;
            }
        };
        log::debug!("Received Telegram Update: {:?}", update);

        if let Some(chat) = update.chat() {
            let chat_id = chat.id.to_string();
            let cached_recipient = recipient_cache.entry(chat_id.clone()).or_insert_with(|| {
                match recipient_repo
                    .find_one_by_to_address_and_type(&chat_id, NotificationType::Telegram)
                {
                    Ok(Some(recipient)) => recipient,
                    Ok(None) => blank_telegram_recipient(),
                    Err(e) => {
                        log::error!("Error looking up recipient in database {}", e);
                        // Because we are returning a new UUID for this telegram chat id, when there's a db error, there is a potential bug here!
                        // With sqlite, it's likely that the whole service will have problems if we can't re-write from the database but we log a warning to restart at least.
                        // Handling this error nicely would required more complex code, making it hard to read and reason about.
                        // Adding caching to the repository layer, or pre-populating the cache at start up might also be a good solution.
                        log::error!("WARNING: After a database error the cache may be inconsistent, please restart the server!!! {}", e);
                        blank_telegram_recipient()
                    }
                }
            });

            // Check if we need to update the recipient name (e.g if the chat title has changed or if we just created the recipient)
            if cached_recipient.name != chat.name() {
                log::debug!(
                    "Chat title doesn't match recipient name, updating recipient: {:?}",
                    chat
                );
                cached_recipient.to_address = chat.id.to_string();
                cached_recipient.name = chat.name();

                let new_recipient = CreateRecipient {
                    id: cached_recipient.id.clone(),
                    name: cached_recipient.name.clone(),
                    notification_type: NotificationType::Telegram,
                    to_address: cached_recipient.to_address.clone(),
                };

                match upsert_recipient(&ctx, new_recipient) {
                    Ok(recipient_result) => log::info!("Updated recipient: {:?}", recipient_result),
                    Err(e) => {
                        log::error!("Error updating recipient, skipping... {:?}", e);
                        log::error!("WARNING: After a database error the cache may be inconsistent, please restart the server!!!");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all, NotificationType};
    use telegram::{TelegramMessage, TelegramUpdate, TelegramUser};
    use util::uuid::uuid;

    use crate::{
        recipient::{self, create::CreateRecipient},
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    const ASYNC_WAIT_MS: u64 = 10;

    #[actix_rt::test]
    async fn test_handle_telegram_updates() {
        let (_mock_data, _, connection_manager, _) =
            setup_all("test_handle_telegram_updates", MockDataInserts::none()).await;

        let (tx, _rx) = tokio::sync::broadcast::channel(10);

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager.clone(),
            get_test_settings(""),
        ));

        let receive_context = ServiceContext::new(service_provider.clone()).unwrap();
        let send_ctx = ServiceContext::new(service_provider.clone()).unwrap();

        let send_channel = tx.clone();
        let update_handler = actix_rt::spawn(async move {
            super::handle_telegram_updates(receive_context, &send_channel).await;
        });

        // Test things don't break if we have an empty update to process (e.g. no chat)

        let empty_update = TelegramUpdate {
            update_id: 1,
            message: None,
            my_chat_member: None,
        };

        tx.send(empty_update).unwrap();

        // wait 10ms to allow processing to happen
        tokio::time::sleep(tokio::time::Duration::from_millis(ASYNC_WAIT_MS)).await;

        // Should be no new recipients...
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, None, None)
            .unwrap();
        assert_eq!(recipients.count, 0);

        let message_update = TelegramUpdate {
            update_id: 2,
            message: Some(TelegramMessage {
                message_id: 1,
                from: TelegramUser {
                    id: 1,
                    is_bot: false,
                    username: None,
                    ..Default::default()
                },
                chat: telegram::TelegramChat {
                    id: 1234,
                    title: Some("telegram_chat_name".to_string()),
                    r#type: "group".to_string(),
                    ..Default::default()
                },
                text: None,
            }),
            my_chat_member: None,
        };

        tx.send(message_update).unwrap();

        // wait 10ms to allow processing to happen
        tokio::time::sleep(tokio::time::Duration::from_millis(ASYNC_WAIT_MS)).await;

        // Should now be 1 recipient...
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, None, None)
            .unwrap();
        assert_eq!(recipients.count, 1);

        // Check we update the title if it changes with a message
        let title_change_update = TelegramUpdate {
            update_id: 3,
            message: Some(TelegramMessage {
                message_id: 1,
                from: TelegramUser {
                    id: 1,
                    is_bot: false,
                    username: None,
                    ..Default::default()
                },
                chat: telegram::TelegramChat {
                    id: 1234,
                    title: Some("telegram_chat_name_changed".to_string()),
                    r#type: "group".to_string(),
                    ..Default::default()
                },
                text: None,
            }),
            my_chat_member: None,
        };

        tx.send(title_change_update).unwrap();

        // wait 10ms to allow processing to happen
        tokio::time::sleep(tokio::time::Duration::from_millis(ASYNC_WAIT_MS)).await;

        // Should still be only 1 recipient
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, None, None)
            .unwrap();
        assert_eq!(recipients.count, 1);

        // Should be able to find the recipient using the new name
        let recipient_filter =
            recipient::RecipientFilter::new().search("telegram_chat_name_changed".to_string());
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, Some(recipient_filter), None)
            .unwrap();
        assert_eq!(recipients.count, 1);

        // check we update the name when we receive a chat member update
        let title_change_update = TelegramUpdate {
            update_id: 3,
            message: None,
            my_chat_member: Some(telegram::TelegramMyChatMember {
                chat: telegram::TelegramChat {
                    id: 1234,
                    title: Some("telegram_chat_name_changed_again".to_string()),
                    r#type: "group".to_string(),
                    ..Default::default()
                },
                from: TelegramUser {
                    id: 1,
                    is_bot: false,
                    username: None,
                    ..Default::default()
                },
            }),
        };

        tx.send(title_change_update).unwrap();

        // wait 10ms to allow processing to happen
        tokio::time::sleep(tokio::time::Duration::from_millis(ASYNC_WAIT_MS)).await;

        // Should still be only 1 recipient
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, None, None)
            .unwrap();
        assert_eq!(recipients.count, 1);

        // Should be able to find the recipient using the new name
        let recipient_filter = recipient::RecipientFilter::new()
            .search("telegram_chat_name_changed_again".to_string());
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, Some(recipient_filter), None)
            .unwrap();
        assert_eq!(recipients.count, 1);

        // create a telegram recipient and put it in the db
        let telegram_recipient = CreateRecipient {
            id: uuid(),
            name: "Notification Group 1".to_string(),
            to_address: "-9999".to_string(),
            notification_type: NotificationType::Telegram,
        };

        let result = service_provider
            .recipient_service
            .create_recipient(&send_ctx, telegram_recipient.clone());

        assert!(result.is_ok());

        // Test that we correctly update the receipt when we receive a message
        let message_update = TelegramUpdate {
            update_id: 4,
            message: Some(TelegramMessage {
                message_id: 1,
                from: TelegramUser {
                    id: 1,
                    is_bot: false,
                    username: None,
                    ..Default::default()
                },
                chat: telegram::TelegramChat {
                    id: -9999,
                    title: Some("Notification Group 1a".to_string()),
                    r#type: "group".to_string(),
                    ..Default::default()
                },
                text: None,
            }),
            my_chat_member: None,
        };

        tx.send(message_update).unwrap();

        // wait 10ms to allow processing to happen
        tokio::time::sleep(tokio::time::Duration::from_millis(ASYNC_WAIT_MS)).await;

        let filter = recipient::RecipientFilter::new().search("Notification Group 1a".to_string());
        let recipients = service_provider
            .recipient_service
            .get_recipients(&send_ctx, None, Some(filter), None)
            .unwrap();
        assert_eq!(recipients.count, 1);
        assert_eq!(recipients.rows[0].name, "Notification Group 1a");

        update_handler.abort();
    }
}
