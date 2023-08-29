use crate::types::TelegramMessageResponse;
use async_graphql::*;
use async_graphql::{Context, Object};
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct TelegramMutations;

#[Object]
impl TelegramMutations {
    async fn send_test_telegram_message(
        &self,
        ctx: &Context<'_>,
        chat_id: String,
    ) -> Result<TelegramMessageResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_ctx = ctx.service_context(Some(&user))?;

        let notification_service = &service_ctx.service_provider.notification_service;

        let html = notification_service
            .render_no_params("test_message/telegram.html")
            .map_err(|e| format!("Unable to render `test_message/telegram.html` : {:?}", e))
            .map_err(StandardGraphqlError::from_string)?;

        let telegram_service = &service_ctx
            .service_provider
            .telegram
            .as_ref()
            .ok_or("Telegram service not configured")
            .map_err(StandardGraphqlError::from_str)?;

        let message = telegram_service
            .send_html_message(&chat_id, &html)
            .await
            .map_err(|e| format!("Unable to send message : {:?}", e))
            .map_err(StandardGraphqlError::from_string)?;

        Ok(TelegramMessageResponse::Response(message.into()))
    }
}
