use async_graphql::{SimpleObject, Union};

#[derive(Union)]
pub enum TelegramMessageResponse {
    Response(TelegramMessageNode),
}

#[derive(PartialEq, Debug, SimpleObject)]
pub struct TelegramMessageNode {
    pub msg_json: String,
}
