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

        let html = notification_service.render_no_params("test_message/telegram.html");
        let html = match html {
            Ok(html) => html,
            Err(err) => {
                return Err(StandardGraphqlError::InternalError(format!(
                    "Unable to render `test_message/telegram.html` : {:?}",
                    err
                ))
                .extend())
            }
        };

        let telegram_service = &service_ctx.service_provider.telegram;

        match telegram_service {
            Some(telegram_service) => {
                let message = telegram_service.send_html_message(&chat_id, &html).await;
                match message {
                    Ok(message) => return Ok(TelegramMessageResponse::Response(message.into())),
                    Err(err) => {
                        return Err(StandardGraphqlError::InternalError(format!(
                            "Unable to send message : {:?}",
                            err
                        ))
                        .extend())
                    }
                }
            }
            None => {
                return Err(StandardGraphqlError::InternalError(
                    "Telegram service not configured".to_string(),
                )
                .extend())
            }
        }
    }
}
