// The responder should reply to the following...
//
// Direct messages to the bot
// Messages to the bot with /hello or /chatid
// Messages to the bot with /help

use crate::{TelegramClient, TelegramUpdate};

pub async fn handle_telegram_updates(
    client: TelegramClient,
    tx_updates: tokio::sync::broadcast::Sender<TelegramUpdate>,
) {
    let mut rx = tx_updates.subscribe();
    loop {
        let result = rx.recv().await;
        let update = match result {
            Ok(update) => {
                log::info!("Received Telegram Update Ready to respond: {:?}", update);
                update
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => return,
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                log::error!("Telegram update lagged behind, something is delaying processing off telegram events!");
                continue;
            }
        };

        // Respond to messages
        match update.message {
            Some(message) => {
                log::debug!("Received Telegram Message Ready to respond: {:?}", message);
                let chat_id = message.chat.id.to_string();
                let text = message.text.unwrap_or_default();

                if text.contains("/hello") || text.contains("/chat") {
                    //TODO: Render template?
                    let response = format!("Hello! This chat_id is {}", chat_id);
                    match client.send_html_message(&chat_id, &response).await {
                        Ok(_) => log::debug!("Sent hello/chat message to chat id {}", chat_id),
                        Err(_) => {
                            log::error!("Failed to send hello message to chat id {}", chat_id)
                        }
                    }
                } else if text == "/help" {
                    //TODO: Render template?
                    let response = format!("I can respond to the following commands:\n/hello - I will say hello to you\n/chatid - I will tell you your chat id\n/help - I will show you this help message");
                    match client.send_html_message(&chat_id, &response).await {
                        Ok(_) => log::debug!("Sent help message to chat id {}", chat_id),
                        Err(_) => log::error!("Failed to send help message to chat id {}", chat_id),
                    }
                }
            }
            None => {
                log::debug!("No message received!: {:?}", update);
            }
        }
    }
}
