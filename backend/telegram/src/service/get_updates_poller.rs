use service::service_provider::ServiceContext;

use crate::TelegramClient;

static TELEGRAM_POLL_TIMEOUT: u32 = 60;

pub async fn get_updates_poller(telegram_client: TelegramClient) {
    loop {
        let updates = service_context
            .service_provider
            .telegram
            .get_updates(&service_context);
        match get_updates {
            Ok(num) => {
                if num > 0 {
                    log::info!("Got {} updates", num);
                }
            }
            Err(error) => log::error!("Error getting updates: {:?}", error),
        }
    }
}
