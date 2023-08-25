use std::sync::Arc;

use datasource::PostgresSettings;
use repository::{
    mock::MockDataInserts,
    test_db::{get_test_db_settings, setup_all},
    StorageConnectionManager,
};
use util::uuid::uuid;

use crate::{
    email::{EmailServiceError, EmailServiceTrait},
    service_provider::{ServiceContext, ServiceProvider},
    settings::{MailSettings, ServerSettings, Settings, TelegramSettings},
};

pub async fn get_test_service_context(mock_data: MockDataInserts) -> ServiceContext {
    let (_, _, connection_manager, _) = setup_all(&uuid(), mock_data).await;

    let service_provider = Arc::new(ServiceProvider::new(
        connection_manager,
        get_test_settings(""),
    ));
    let context = ServiceContext::new(service_provider).unwrap();

    context
}

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
        datasource: PostgresSettings {
            username: String::new(),
            password: String::new(),
            port: 0,
            host: String::new(),
            database_name: String::new(),
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
