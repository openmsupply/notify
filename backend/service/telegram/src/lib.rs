struct TelegramClient {
    token: String,
    http_client: reqwest::Client,
    base_url: String,
}

#[derive(Debug)]
enum TemporaryErrorType {
    TimedOut(String),
    TooManyRequests,
    InternalServerError(String),
    Other(String),
}

#[derive(Debug)]
enum TelegramError {
    Fatal(String),
    Temporary(TemporaryErrorType),
}

impl From<reqwest::Error> for TelegramError {
    fn from(error: reqwest::Error) -> TelegramError {
        // TODO, revisit fatal non fatal errors
        if error.is_timeout() {
            return TelegramError::Temporary(TemporaryErrorType::TimedOut(error.to_string()));
        }
        if let Some(status) = error.status() {
            match status {
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    return TelegramError::Temporary(TemporaryErrorType::TooManyRequests);
                }
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
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
    fn new(token: String) -> TelegramClient {
        let http_client = reqwest::Client::new();
        let url = format!("https://api.telegram.org/bot{}", token);
        TelegramClient {
            token: token,
            http_client: http_client,
            base_url: url,
        }
    }

    async fn send_message(&self, chat_id: &str, message: &str) -> Result<(), TelegramError> {
        let params = [("chat_id", chat_id), ("text", message)];
        let url = format!("{}/sendMessage", self.base_url);

        let response = self.http_client.post(&url).form(&params).send().await?;

        let response_text = response.text().await?;

        dbg!(response_text);
        Ok(())
    }

    async fn send_html_message(&self, chat_id: &str, html: &str) -> Result<(), TelegramError> {
        let params = [("chat_id", chat_id), ("text", html), ("parse_mode", "HTML")];
        let url = format!("{}/sendMessage", self.base_url);

        let response = self.http_client.post(&url).form(&params).send().await?;
        let response_text = response.text().await?;

        dbg!(response_text);
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "telegram-tests")]
mod test {
    use super::*;

    fn get_telegram_token_from_env() -> String {
        std::env::var("TELEGRAM_TOKEN").unwrap()
    }

    fn get_telegram_chat_id_from_env() -> String {
        std::env::var("TELEGRAM_CHAT_ID").unwrap()
    }

    #[tokio::test]
    async fn test_send_message() {
        let client = TelegramClient::new(get_telegram_token_from_env());
        client
            .send_message(&get_telegram_chat_id_from_env(), "Hello from notify")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_send_html_message() {
        let client = TelegramClient::new(get_telegram_token_from_env());
        client
            .send_html_message(
                &get_telegram_chat_id_from_env(),
                "<a href=\"https://www.msupply.foundation\">Visit the mSupply Foundation Website</a>",
            )
            .await
            .unwrap();
    }
}
