use crate::TelegramClient;
use log;

static TELEGRAM_POLL_TIMEOUT_SECONDS: i64 = 30;
static ERROR_SLEEP_SECONDS: u64 = 10;

pub async fn get_updates_poller(telegram_client: &TelegramClient) {
    // Might be a good idea to persist this to the KV store so we can pickup from where we left off on a restart?
    let mut last_update_id: i64 = -2;

    loop {
        let updates = telegram_client
            .get_updates(last_update_id, TELEGRAM_POLL_TIMEOUT_SECONDS)
            .await;
        match updates {
            Ok(updates) => {
                let num = updates.len();
                log::debug!("Got {} updates", num);
                for update in updates {
                    let update_id = match update.get("update_id") {
                        Some(update_id) => match update_id.as_i64() {
                            Some(update_id) => update_id,
                            None => {
                                log::error!("update_id is not an i64: {:?}", update_id);
                                continue;
                            }
                        },
                        None => {
                            log::error!("No update_id in update: {:?}", update);
                            continue;
                        }
                    };
                    if update_id > last_update_id {
                        last_update_id = update_id;
                    }
                    log::debug!("Got update: {:?}", update);
                }
            }

            Err(error) => {
                log::error!("Error getting updates: {:?} \n Sleeping...", error);
                // Sleep for a bit so we don't hammer the CPU or telegram API
                tokio::time::sleep(std::time::Duration::from_secs(ERROR_SLEEP_SECONDS)).await;
            }
        };
    }
}
