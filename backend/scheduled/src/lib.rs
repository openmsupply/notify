use service::{
    plugin::{PluginError, PluginTrait},
    service_provider::{ServiceContext, ServiceProvider},
};
use std::sync::Arc;

pub mod process;

const MIN_SLEEP_TIME_MS: u64 = 30 * 1000; // 30 seconds

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
        loop {
            // Process any scheduled notifications that are due
            let current_time = chrono::Utc::now().naive_utc();
            let result = process::process_scheduled_notifications(&service_context, current_time);
            match result {
                Ok(_) => {
                    log::info!("Successfully processed scheduled notifications");
                }
                Err(e) => {
                    log::error!("Error processing scheduled notifications: {:?}", e);
                }
            }

            // Sleep for a while before checking for any notifications due again
            std::thread::sleep(std::time::Duration::from_millis(MIN_SLEEP_TIME_MS));
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
