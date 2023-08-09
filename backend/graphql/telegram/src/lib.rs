pub mod mutations;
pub mod types;
pub use mutations::*;

use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct TelegramQueries;

#[Object]
impl TelegramQueries {
    #[allow(non_snake_case)]
    pub async fn telegram_bot_name(&self, ctx: &Context<'_>) -> Result<String> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_ctx = ctx.service_context(Some(&user))?;
        let telegram_service = &ctx.service_provider().telegram;

        let bot_name = match telegram_service {
            Some(telegram_service) => {
                let message = telegram_service.get_name().await;
                match message {
                    Ok(message) => message,
                    Err(err) => {
                        format!("Unable to get botname : {:?}", err)
                    }
                }
            }
            None => "Telegram service not configured".to_string(),
        };
        Ok(bot_name)
    }
}
