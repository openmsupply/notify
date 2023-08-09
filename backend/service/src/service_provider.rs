use std::sync::Arc;

use repository::{RepositoryError, StorageConnection, StorageConnectionManager};
use telegram::TelegramClient;

use crate::{
    auth::{AuthService, AuthServiceTrait},
    email::{EmailService, EmailServiceTrait},
    recipient::{RecipientService, RecipientServiceTrait},
    settings::Settings,
    user_account::{UserAccountService, UserAccountServiceTrait},
};

pub struct ServiceProvider {
    pub connection_manager: StorageConnectionManager,
    pub email_service: Box<dyn EmailServiceTrait>,
    pub validation_service: Box<dyn AuthServiceTrait>,
    pub user_account_service: Box<dyn UserAccountServiceTrait>,
    pub recipient_service: Box<dyn RecipientServiceTrait>,
    pub settings: Settings,
    pub telegram: Option<TelegramClient>,
}

pub struct ServiceContext {
    pub connection: StorageConnection,
    // pub service_provider: Arc<ServiceProvider>,
    pub user_id: String,
    pub app_url: String,
}

impl ServiceContext {
    pub fn new(service_provider: Arc<ServiceProvider>) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            user_id: "".to_string(),
            app_url: service_provider.settings.server.app_url.clone(),
        })
    }

    pub fn with_user(
        service_provider: Arc<ServiceProvider>,
        user_id: String,
    ) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            user_id,
            app_url: service_provider.settings.server.app_url.clone(),
        })
    }

    pub fn as_server_admin(
        service_provider: Arc<ServiceProvider>,
    ) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            // service_provider,
            user_id: "9cd8ce10-969b-45c4-871e-3a744c75ddf0".to_string(), // Admin user id is hardcoded in the database migration
            app_url: service_provider.settings.server.app_url.clone(),
        })
    }
}

impl ServiceProvider {
    pub fn new(connection_manager: StorageConnectionManager, settings: Settings) -> Self {
        let telegram = match &settings.telegram.token {
            Some(token) => Some(TelegramClient::new(token.clone())),
            None => None,
        };

        ServiceProvider {
            connection_manager,
            email_service: Box::new(EmailService::new(settings.clone())),
            validation_service: Box::new(AuthService::new()),
            user_account_service: Box::new(UserAccountService {}),
            recipient_service: Box::new(RecipientService {}),
            settings: settings,
            telegram: telegram,
        }
    }

    /// Establishes a new DB connection
    pub fn connection(&self) -> Result<StorageConnection, RepositoryError> {
        self.connection_manager.connection()
    }
}
