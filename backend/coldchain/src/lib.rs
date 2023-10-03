use service::{
    plugin::{PluginError, PluginTrait},
    service_provider::ServiceContext,
};

pub mod alerts;

pub struct ColdChainPlugin {}

impl PluginTrait for ColdChainPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        ColdChainPlugin {}
    }

    fn name(&self) -> String {
        "ColdChain".to_string()
    }

    fn tick(&self, _ctx: &ServiceContext) -> Result<(), PluginError> {
        log::debug!("Running ColdChainPlugin");
        log::error!("COLD CHAIN NOT YET IMPLEMENTED");
        Ok(())
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
    async fn cold_chain_plugin_has_a_name() {
        let plugin = ColdChainPlugin::new();
        assert_eq!(plugin.name(), "ColdChain");
    }

    #[tokio::test]
    async fn cold_chain_plugin_can_tick() {
        let (_, _, connection_manager, _) =
            setup_all("cold_chain_plugin_can_start", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let ctx = ServiceContext::as_server_admin(service_provider).unwrap();

        let plugin = ColdChainPlugin::new();
        let result = plugin.tick(&ctx);
        assert!(result.is_ok());
    }
}
