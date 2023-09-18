use std::sync::Arc;

use repository::{EqualFilter, NotificationConfigFilter, NotificationConfigKind, PaginationOption};
use service::{
    plugin::{PluginError, PluginTrait},
    service_provider::{ServiceContext, ServiceProvider},
};

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

    fn start(&self) -> Result<(), PluginError> {
        log::info!("Starting ScheduledNotificationPlugin");

        // Create a service context
        let service_context = ServiceContext::new(self.service_provider.clone()).unwrap();

        // Find all the scheduled notification configs that need to be processed

        let filter = NotificationConfigFilter::new().kind(EqualFilter::equal_to_generic(
            NotificationConfigKind::Scheduled,
        ));

        // TODO: Implement pagination
        let scheduled_notification_configs = self
            .service_provider
            .notification_config_service
            .get_notification_configs(
                &service_context,
                Some(PaginationOption {
                    limit: Some(1000),
                    offset: Some(0),
                }),
                Some(filter),
                None,
            )
            .map_err(|e| PluginError::PluginFailedToStart(format!("{:?}", e)))?;

        // print the names of the scheduled notification configs
        for scheduled_notification_config in scheduled_notification_configs.rows {
            log::info!(
                "ScheduledNotificationPlugin loaded: {}",
                scheduled_notification_config.title
            );
        }
        loop {
            // Check if any scheduled notifications are due

            // Load the notification config

            // Run SQL Queries to get the data

            // Put sql queries data into Json Value for template

            // Send the notification
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
