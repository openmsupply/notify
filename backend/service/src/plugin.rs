use std::sync::Arc;

use crate::service_provider::ServiceProvider;

#[derive(Debug)]
pub enum PluginError {
    UnableToProcessTick(String),
}

pub trait PluginTrait: Send + Sync {
    fn new(service_provider: Arc<ServiceProvider>) -> Self
    where
        Self: Sized;
    fn name(&self) -> String;

    fn tick(&self) -> Result<(), PluginError> {
        // Plugins should process their work here
        Ok(())
    }
}
