use repository::{test_db::get_test_db_settings, StorageConnectionManager};

use crate::{
    email::{EmailServiceError, EmailServiceTrait},
    service_provider::{ServiceContext, ServiceProvider},
    settings::{MailSettings, ServerSettings, Settings, TelegramSettings},
};

// The following settings work for PG and Sqlite (username, password, host and port are
// ignored for the later)
pub fn get_test_settings(db_name: &str) -> Settings {
    Settings {
        server: ServerSettings {
            port: 5432,
            cors_origins: vec!["http://localhost:3007".to_string()],
            base_dir: None,
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
        telegram: TelegramSettings { token: None },
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

    #[cfg(feature = "telegram-tests")]
    pub fn send_test_notifications(context: &ServiceContext) {
        context
            .service_provider
            .notification_service
            .send_queued_notifications(&context)
            .unwrap();
    }

    #[cfg(not(feature = "telegram-tests"))]
    pub fn send_test_notifications(_context: &ServiceContext) {
        println!("Skipping notification sending");
    }
}
