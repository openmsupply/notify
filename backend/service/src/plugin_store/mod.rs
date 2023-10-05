use repository::PluginStoreRepository;

use crate::service_provider::ServiceContext;

pub trait PluginServiceTrait: Send + Sync {
    fn get_value(
        &self,
        ctx: &ServiceContext,
        plugin_name: String,
        key: String,
    ) -> Result<Option<String>, PluginServiceError>;

    fn set_value(
        &self,
        ctx: &ServiceContext,
        plugin_name: String,
        key: String,
        value: String,
    ) -> Result<(), PluginServiceError>;
}

pub struct PluginService {}

#[derive(Debug)]
pub enum PluginServiceError {
    InternalError(String),
    BadUserInput(String),
}

impl PluginServiceTrait for PluginService {
    fn get_value(
        &self,
        ctx: &ServiceContext,
        plugin_name: String,
        key: String,
    ) -> Result<Option<String>, PluginServiceError> {
        let repository = PluginStoreRepository::new(&ctx.connection);

        let result = repository
            .get_string(plugin_name, key)
            .map_err(|e| PluginServiceError::InternalError(format!("{:?}", e)))?;
        Ok(result)
    }

    fn set_value(
        &self,
        ctx: &ServiceContext,
        plugin_name: String,
        key: String,
        value: String,
    ) -> Result<(), PluginServiceError> {
        let repository = PluginStoreRepository::new(&ctx.connection);

        let _result = repository
            .set_string(plugin_name, key, value)
            .map_err(|e| PluginServiceError::InternalError(format!("{:?}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use repository::{mock::MockDataInserts, test_db::setup_all};

    use crate::{
        service_provider::{ServiceContext, ServiceProvider},
        test_utils::get_test_settings,
    };

    #[actix_rt::test]
    async fn get_value_returns_none_if_no_value() {
        let (_, _, connection_manager, _) = setup_all(
            "get_value_returns_none_if_no_value",
            MockDataInserts::none(),
        )
        .await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let ctx = ServiceContext::new(service_provider.clone()).unwrap();

        let result = service_provider.plugin_service.get_value(
            &ctx,
            "plugin_name".to_string(),
            "key".to_string(),
        );
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[actix_rt::test]
    async fn set_value_sets_value() {
        let (_, _, connection_manager, _) =
            setup_all("set_value_sets_value", MockDataInserts::none()).await;

        let service_provider = Arc::new(ServiceProvider::new(
            connection_manager,
            get_test_settings(""),
        ));
        let ctx = ServiceContext::new(service_provider.clone()).unwrap();

        let plugin_name = "test_plugin".to_string();
        let key = "test_key".to_string();
        let value = "test_value".to_string();

        let result = service_provider.plugin_service.set_value(
            &ctx,
            plugin_name.clone(),
            key.clone(),
            value.clone(),
        );
        assert!(result.is_ok());

        // Get the same key again to see it was set
        let result = service_provider
            .plugin_service
            .get_value(&ctx, plugin_name, key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), value);
    }
}
