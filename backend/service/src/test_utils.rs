use std::{
    env,
    path::{Path, PathBuf},
};

use repository::{test_db::get_test_db_settings, StorageConnectionManager};

use crate::{
    email::{EmailServiceError, EmailServiceTrait},
    service_provider::{ServiceContext, ServiceProvider},
    settings::{MailSettings, ServerSettings, Settings, TelegramSettings},
};

use self::telegram_test::get_telegram_token_from_env;

pub fn find_base_dir() -> PathBuf {
    // Assume the base path is the base path of one of the project crates:
    search_for_base_dir(Path::new(&env::current_dir().unwrap())).unwrap()
}

pub fn search_for_base_dir(path: &Path) -> Result<PathBuf, String> {
    // Strategy is to find the repository crate directory, then assume base path is the only one that contains a folder called repository
    let repository_path = path.join("repository");
    if repository_path.is_dir() {
        Ok(path.to_path_buf())
    } else {
        path.parent()
            .map(search_for_base_dir)
            .unwrap_or_else(|| Err("Failed to locate migrations directory".to_string()))
    }
}

// The following settings work for PG and Sqlite (username, password, host and port are
// ignored for the later)
pub fn get_test_settings(db_name: &str) -> Settings {
    let telegram_token = get_telegram_token_from_env();
    Settings {
        server: ServerSettings {
            port: 5432,
            cors_origins: vec!["http://localhost:3007".to_string()],
            base_dir: Some(find_base_dir().to_str().unwrap().to_string()),
            app_url: "http://localhost:8007".to_string(),
        },
        database: get_test_db_settings(db_name),
        mail: MailSettings {
            port: 1025,
            host: "localhost".to_string(),
            starttls: false,
            username: "".to_string(),
            password: "".to_string(),
            from: "no-reply@msupply.foundation".to_string(),
        },
        telegram: TelegramSettings {
            token: telegram_token,
        },
    }
}

struct MockEmailService {}

impl EmailServiceTrait for MockEmailService {
    fn test_connection(&self) -> Result<bool, EmailServiceError> {
        Ok(true)
    }

    fn send_queued_emails(&self, _ctx: &ServiceContext) -> Result<usize, EmailServiceError> {
        Ok(0)
    }
}

// Create a service provider with a dummy email service
pub fn service_provider_with_mock_email_service(
    connection_manager: &StorageConnectionManager,
) -> ServiceProvider {
    let settings = get_test_settings("db_name"); //because we already have a storage connection manager db_name is not used
    let mut service_provider = ServiceProvider::new(connection_manager.clone(), settings);
    service_provider.email_service = Box::new(MockEmailService {});
    service_provider
}

#[cfg(test)]
pub mod email_test {
    use crate::service_provider::ServiceContext;

    #[cfg(feature = "email-tests")]
    pub fn send_test_emails(context: &ServiceContext) {
        context
            .service_provider
            .email_service
            .send_queued_emails(&context)
            .unwrap();
    }

    #[cfg(not(feature = "email-tests"))]
    pub fn send_test_emails(_context: &ServiceContext) {
        println!("Skipping email sending");
    }
}

pub mod telegram_test {
    use crate::service_provider::ServiceContext;

    pub fn get_default_telegram_chat_id() -> String {
        std::env::var("TELEGRAM_CHAT_ID").expect(
            "Please set the TELEGRAM_CHAT_ID environment variable to run the telegram tests",
        )
    }

    pub fn get_telegram_token_from_env() -> Option<String> {
        match std::env::var("TELEGRAM_TOKEN") {
            Ok(token) => Some(token),
            Err(_) => {
                println!(
                    "Please set the TELEGRAM_TOKEN environment variable to run the telegram tests"
                );
                None
            }
        }
    }

    #[cfg(feature = "telegram-tests")]
    pub async fn send_test_notifications(context: &ServiceContext) {
        context
            .service_provider
            .notification_service
            .send_queued_notifications(&context)
            .await
            .unwrap();
    }

    #[cfg(not(feature = "telegram-tests"))]
    pub async fn send_test_notifications(_context: &ServiceContext) {
        println!("Skipping notification sending");
    }
}
