use std::sync::Arc;

use repository::{RepositoryError, StorageConnection, StorageConnectionManager};
use telegram::TelegramClient;

use crate::{
    auth::{AuthService, AuthServiceTrait},
    datasource::{DatasourceService, DatasourceServiceTrait},
    email::{EmailService, EmailServiceTrait},
    notification_config::{NotificationConfigService, NotificationConfigServiceTrait},
    recipient::{RecipientService, RecipientServiceTrait},
    recipient_list::{RecipientListService, RecipientListServiceTrait},
    settings::Settings,
    user_account::{UserAccountService, UserAccountServiceTrait},
};

pub struct ServiceProvider {
    pub connection_manager: StorageConnectionManager,
    pub datasource_service: Box<dyn DatasourceServiceTrait>,
    pub email_service: Box<dyn EmailServiceTrait>,
    pub validation_service: Box<dyn AuthServiceTrait>,
    pub user_account_service: Box<dyn UserAccountServiceTrait>,
    pub notification_config_service: Box<dyn NotificationConfigServiceTrait>,
    pub recipient_service: Box<dyn RecipientServiceTrait>,
    pub recipient_list_service: Box<dyn RecipientListServiceTrait>,
    pub settings: Settings,
    pub telegram: Option<TelegramClient>,
}

pub struct ServiceContext {
    pub connection: StorageConnection,
    pub service_provider: Arc<ServiceProvider>,
    pub user_id: String,
}

impl ServiceContext {
    pub fn new(service_provider: Arc<ServiceProvider>) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            service_provider,
            user_id: "".to_string(),
        })
    }

    pub fn with_user(
        service_provider: Arc<ServiceProvider>,
        user_id: String,
    ) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            service_provider,
            user_id,
        })
    }

    pub fn as_server_admin(
        service_provider: Arc<ServiceProvider>,
    ) -> Result<ServiceContext, RepositoryError> {
        let connection = service_provider.connection_manager.connection()?;
        Ok(ServiceContext {
            connection,
            service_provider,
            user_id: "9cd8ce10-969b-45c4-871e-3a744c75ddf0".to_string(), // Admin user id is hardcoded in the database migration
        })
    }
}

impl ServiceProvider {
    pub fn new(connection_manager: StorageConnectionManager, settings: Settings) -> Self {
        let telegram = match &settings.telegram.token {
            Some(token) => Some(TelegramClient::new(token.clone())),
            None => None,
        };

        //TODO Rearrange this so we're not using port number to determine if we want to mock...
        // Should move this logic somehow into the tests...
        let datasource_service: Box<dyn DatasourceServiceTrait> = match &settings.datasource.port {
            0 => Box::new(crate::test_utils::MockDatasourceService {}),
            _ => Box::new(DatasourceService::new(settings.clone())),
        };

        ServiceProvider {
            connection_manager,
            email_service: Box::new(EmailService::new(settings.clone())),
            datasource_service: datasource_service,
            validation_service: Box::new(AuthService::new()),
            user_account_service: Box::new(UserAccountService {}),
            notification_config_service: Box::new(NotificationConfigService {}),
            recipient_service: Box::new(RecipientService {}),
            recipient_list_service: Box::new(RecipientListService {}),
            settings: settings,
            telegram: telegram,
        }
    }

    /// Establishes a new DB connection
    pub fn connection(&self) -> Result<StorageConnection, RepositoryError> {
        self.connection_manager.connection()
    }
}
