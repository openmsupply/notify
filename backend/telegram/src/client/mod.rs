use http::StatusCode;
use serde::Serialize;
use serde_json::{self, Value};
use std::time::Duration;

use crate::{TelegramApiResponse, TelegramChat, TelegramMessage};

const DEFAULT_REQUEST_TIMEOUT: u64 = 60;

#[derive(Clone)]
pub struct TelegramClient {
    http_client: reqwest::Client,
    base_url: String,
}

#[derive(Debug)]
pub enum TemporaryErrorType {
    TimedOut(String),
    ConnectionError(String),
    InternalServerError(String),
    Flood(String),
    Other(String),
}

#[derive(Debug)]
pub enum TelegramError {
    Fatal(String),
    Temporary(TemporaryErrorType),
}

#[derive(Serialize)]
struct GetUpdatesParams {
    offset: Option<i64>,
    timeout: i64,
}

impl From<reqwest::Error> for TelegramError {
    fn from(error: reqwest::Error) -> TelegramError {
        if error.is_timeout() {
            return TelegramError::Temporary(TemporaryErrorType::TimedOut(error.to_string()));
        }
        if error.is_connect() {
            return TelegramError::Temporary(TemporaryErrorType::ConnectionError(
                error.to_string(),
            ));
        }

        // https://core.telegram.org/api/errors
        if let Some(status) = error.status() {
            // Flood errors mean we should wait before creating new requests
            if status == StatusCode::from_u16(420).unwrap_or_default() {
                return TelegramError::Temporary(TemporaryErrorType::Flood(error.to_string()));
            }
            match status {
                StatusCode::INTERNAL_SERVER_ERROR => {
                    return TelegramError::Temporary(TemporaryErrorType::InternalServerError(
                        error.to_string(),
                    ));
                }
                _ => return TelegramError::Fatal(error.to_string()),
            }
        }
        TelegramError::Fatal(error.to_string())
    }
}

impl TelegramClient {
    pub fn new(token: String) -> TelegramClient {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT))
            .build()
            .expect("Something went unexpectedly wrong building the telegram reqwest client");
        let url = format!("https://api.telegram.org/bot{}", token);
        TelegramClient {
            http_client: http_client,
            base_url: url,
        }
    }

    pub async fn get_name(&self) -> Result<String, TelegramError> {
        let url = format!("{}/getMyName", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        let response_text = response.text().await?;

        let telegram_response: TelegramApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| TelegramError::Fatal(e.to_string()))?;

        if !telegram_response.ok {
            return Err(TelegramError::Fatal(response_text));
        }

        telegram_response
            .result
            .get("name")
            .ok_or_else(|| TelegramError::Fatal("No name in response".to_string()))?
            .as_str()
            .ok_or_else(|| TelegramError::Fatal("name is not a string".to_string()))
            .map(|s| s.to_string())
    }

    pub async fn get_chat(&self, chat_id: &str) -> Result<TelegramChat, TelegramError> {
        let params = [("chat_id", chat_id)];
        let url = format!("{}/getChat", self.base_url);

        let response = self.http_client.post(&url).form(&params).send().await?;
        let response_text = response.text().await?;

        let telegram_response: TelegramApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| TelegramError::Fatal(e.to_string()))?;

        if !telegram_response.ok {
            return Err(TelegramError::Fatal(response_text));
        }

        let chat: TelegramChat = serde_json::from_value(telegram_response.result)
            .map_err(|e| TelegramError::Fatal(e.to_string()))?;

        Ok(chat)
    }

    pub async fn send_html_message(
        &self,
        chat_id: &str,
        html: &str,
    ) -> Result<TelegramMessage, TelegramError> {
        let params = [("chat_id", chat_id), ("text", html), ("parse_mode", "HTML")];
        let url = format!("{}/sendMessage", self.base_url);

        let response = self.http_client.post(&url).form(&params).send().await?;
        let response_text = response.text().await?;

        let telegram_response: TelegramApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| TelegramError::Fatal(format!("{}-{}", e.to_string(), response_text)))?;

        if !telegram_response.ok {
            return Err(TelegramError::Fatal(response_text));
        }

        let message: TelegramMessage = serde_json::from_value(telegram_response.result)
            .map_err(|e| TelegramError::Fatal(format!("Unable to interpret message - {:?}", e)))?;

        Ok(message)
    }

    /// last_update_id +1 maps to "offset" parameter, api description of offset:
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers
    /// of previously received updates. By default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id.
    pub async fn get_updates(
        &self,
        last_confirmed_id: Option<i64>,
        timeout: i64,
    ) -> Result<Vec<Value>, TelegramError> {
        let url = format!("{}/getUpdates", self.base_url);
        // We add one to the last update_id so we don't get the same updates again
        let params = GetUpdatesParams {
            offset: last_confirmed_id.map(|id| id + 1),
            timeout,
        };

        let response = self.http_client.get(&url).form(&params).send().await?;
        let response_text = response.text().await?;

        log::debug!("Response from telegram: {}", response_text);

        let telegram_response: TelegramApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| TelegramError::Fatal(e.to_string()))?;

        if !telegram_response.ok {
            return Err(TelegramError::Fatal(response_text));
        }

        let updates: Vec<Value> = serde_json::from_value(telegram_response.result)
            .map_err(|e| TelegramError::Fatal(e.to_string()))?;

        Ok(updates)
    }
}

#[cfg(test)]
#[cfg(feature = "telegram-tests")]
mod test {
    use super::*;

    fn get_telegram_token_from_env() -> String {
        std::env::var("TELEGRAM_TOKEN")
            .expect("Please set the TELEGRAM_TOKEN environment variable to run the telegram tests")
    }

    fn get_telegram_chat_id_from_env() -> String {
        std::env::var("TELEGRAM_CHAT_ID").expect(
            "Please set the TELEGRAM_CHAT_ID environment variable to run the telegram tests",
        )
    }

    #[tokio::test]
    async fn test_get_name() {
        let client = TelegramClient::new(get_telegram_token_from_env());
        let name = client.get_name().await;
        if !name.is_ok() {
            println!(
                "Unable to get name of bot, your environment might not be setup correctly: {:?}",
                name
            );
        }

        assert!(name.is_ok());
        println!("My name is {}", name.unwrap());
    }

    #[tokio::test]
    async fn test_get_chat() {
        let client = TelegramClient::new(get_telegram_token_from_env());
        let chat = client.get_chat(&get_telegram_chat_id_from_env()).await;
        if !chat.is_ok() {
            println!(
                "Unable to get chat, your environment might not be setup correctly: {:?}",
                chat
            );
        }

        assert!(chat.is_ok());
        println!("Chat is {:?}", chat.unwrap());
    }

    #[tokio::test]
    async fn test_send_html_message() {
        let client = TelegramClient::new(get_telegram_token_from_env());
        client
            .send_html_message(
                &get_telegram_chat_id_from_env(),
                "This is a test message from Notify. Find out more by about notify by <a href=\"https://www.msupply.foundation\">Visiting the mSupply Foundation Website</a>",
            )
            .await
            .unwrap();
    }
}
