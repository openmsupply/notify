use service::{
    plugin::{PluginError, PluginTrait},
    service_provider::ServiceContext,
};

pub mod parse;
pub mod process;
pub mod query;

#[derive(Debug)]
pub enum NotificationError {
    InvalidTemplate,
    InvalidRecipient,
    UnableToParseConfig(String),
    InternalError(String),
    InvalidNextDueDate,
}

pub struct ScheduledNotificationPlugin {}

impl PluginTrait for ScheduledNotificationPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        ScheduledNotificationPlugin {}
    }

    fn name(&self) -> String {
        "ScheduledNotification".to_string()
    }

    fn tick(&self, ctx: &ServiceContext) -> Result<(), PluginError> {
        log::debug!("Running ScheduledNotificationPlugin");

        // Process any scheduled notifications that are due
        let current_time = chrono::Utc::now().naive_utc();
        let result = process::process_scheduled_notifications(ctx, current_time);
        match result {
            Ok(count) => {
                if count > 0 {
                    log::info!("Successfully processed {} scheduled notifications", count);
                }
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
    async fn scheduled_plugin_has_a_name() {
        let plugin = ScheduledNotificationPlugin::new();
        assert_eq!(plugin.name(), "ScheduledNotification");
    }

    #[tokio::test]
    async fn scheduled_plugin_can_tick() {
        let (_, _, connection_manager, _) =
            setup_all("scheduled_plugin_can_start", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let ctx = ServiceContext::as_server_admin(service_provider).unwrap();

        let plugin = ScheduledNotificationPlugin::new();
        let result = plugin.tick(&ctx);
        assert!(result.is_ok());
    }
}
