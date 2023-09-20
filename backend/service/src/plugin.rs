use crate::service_provider::ServiceContext;

#[derive(Debug)]
pub enum PluginError {
    UnableToProcessTick(String),
}

pub trait PluginTrait: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn name(&self) -> String;

    fn tick(&self, _ctx: &ServiceContext) -> Result<(), PluginError> {
        // Plugins should process their work here
        Ok(())
    }
}
