use std::sync::Arc;

use service::{plugin::PluginTrait, service_provider::ServiceProvider};

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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};
    use service::test_utils::get_test_settings;

    use service::service_provider::ServiceProvider;

    use super::*;

    #[tokio::test]
    async fn it_works() {
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
