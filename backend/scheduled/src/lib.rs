use service::{
    plugin::{PluginError, PluginTrait},
    service_provider::{ServiceContext, ServiceProvider},
};
use std::sync::Arc;

pub mod parse;
pub mod process;

#[derive(Debug)]
pub enum NotificationError {
    InvalidTemplate,
    InvalidRecipient,
    UnableToParseConfig(String),
    InternalError(String),
    InvalidNextDueDate,
}

pub struct ScheduledNotificationPlugin {
    service_provider: Arc<ServiceProvider>,
}

impl PluginTrait for ScheduledNotificationPlugin {
    fn new(service_provider: Arc<ServiceProvider>) -> Self
    where
        Self: Sized,
    {
        ScheduledNotificationPlugin { service_provider }
    }

    fn name(&self) -> String {
        "ScheduledNotification".to_string()
    }

    fn tick(&self) -> Result<(), PluginError> {
        log::info!("Starting ScheduledNotificationPlugin");

        // Create a service context
        let service_context = ServiceContext::new(self.service_provider.clone()).unwrap();

        // Process any scheduled notifications that are due
        let current_time = chrono::Utc::now().naive_utc();
        let result = process::process_scheduled_notifications(&service_context, current_time);
        match result {
            Ok(count) => {
                log::info!("Successfully processed {} scheduled notifications", count);
                Ok(())
            }
            Err(e) => {
                log::error!("Error processing scheduled notifications: {:?}", e);
                Err(PluginError::UnableToProcessTick(format!(
                    "Error processing scheduled notifications: {:?}",
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use service::test_utils::get_test_settings;

    use service::service_provider::ServiceProvider;

    use super::*;

    #[tokio::test]
    async fn scheduled_plugin_can_start() {
        let (_, _, connection_manager, _) =
            setup_all("scheduled_plugin_can_start", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let plugin = ScheduledNotificationPlugin::new(service_provider.clone());
        assert_eq!(plugin.name(), "ScheduledNotification");
    }
}
