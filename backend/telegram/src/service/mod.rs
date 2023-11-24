use tokio::sync::broadcast::Sender;

use crate::TelegramClient;

use self::processor::poll_get_updates;

pub mod markdown;
mod processor;
mod responder;

pub struct TelegramService {
    pub client: TelegramClient,
    pub app_url: String,
    pub updates_channel: Sender<crate::TelegramUpdate>,
}

impl TelegramService {
    pub fn new(client: TelegramClient, app_url: String) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(10);
        TelegramService {
            client,
            app_url,
            updates_channel: tx, // we only need the transmission side because consumers just subscribe via this
        }
    }

    pub async fn init(self) -> Sender<crate::TelegramUpdate> {
        // let app_url = self.app_url.clone();
        let responder_channel = self.updates_channel.clone();
        let responder_client = self.client.clone();
        tokio::spawn(async move {
            responder::handle_telegram_updates(responder_client, responder_channel).await;
        });

        let sender_channel = self.updates_channel.clone();
        let sender_client = self.client.clone();
        tokio::spawn(async move {
            poll_get_updates(&sender_client, &sender_channel).await;
        });
        // Return the channel so consumers can subscribe
        self.updates_channel
    }
}
