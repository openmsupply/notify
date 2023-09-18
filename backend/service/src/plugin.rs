use std::sync::Arc;

use crate::service_provider::ServiceProvider;

pub trait PluginTrait: Send + Sync {
    fn new(service_provider: Arc<ServiceProvider>) -> Self
    where
        Self: Sized;
    fn name(&self) -> String;

    fn start(&self) -> Result<(), String> {
        Ok(())
    }
}
