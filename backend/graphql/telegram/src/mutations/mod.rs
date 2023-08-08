use crate::types::{TelegramMessageNode, TelegramMessageResponse};
use async_graphql::*;
use async_graphql::{Context, Object};

#[derive(Default, Clone)]
pub struct TelegramMutations;

#[Object]
impl TelegramMutations {
    async fn send_test_message(
        &self,
        ctx: &Context<'_>,
        chat_id: String,
    ) -> Result<TelegramMessageResponse> {
        Ok(TelegramMessageResponse::Response(TelegramMessageNode {
            msg_json: "{something:1.0}".to_string(),
        }))
    }
}
