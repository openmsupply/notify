use async_graphql::*;
use graphql_core::standard_graphql_error::{validate_auth, StandardGraphqlError};
use service::{
    auth::{Resource, ResourceAccessRequest},
    ListResult,
};

// TODO: Move to telegram service
#[derive(PartialEq, Debug)]
struct TelegramChatRow {
    chat_id: String,
    name: String,
}

#[derive(PartialEq, Debug)]
pub struct TelegramChatNode {
    chat: TelegramChatRow,
}

#[derive(SimpleObject)]
pub struct TelegramChatConnector {
    total_count: u32,
    nodes: Vec<TelegramChatNode>,
}

#[Object]
impl TelegramChatNode {
    pub async fn chat_id(&self) -> &str {
        &self.row().chat_id
    }

    pub async fn name(&self) -> &str {
        &self.row().name
    }
}

impl TelegramChatNode {
    // pub fn from_domain(chat: TelegramChatRow) -> Self {
    //     TelegramChatNode { chat }
    // }

    pub fn row(&self) -> &TelegramChatRow {
        &self.chat
    }
}

// impl TelegramChatConnector {
//     pub fn from_domain(chat_ids: ListResult<TelegramChatRow>) -> TelegramChatConnector {
//         TelegramChatConnector {
//             total_count: chat_ids.count,
//             nodes: chat_ids
//                 .rows
//                 .into_iter()
//                 .map(|chat| TelegramChatRow::from_domain(chat))
//                 .collect(),
//         }
//     }
// }

#[derive(Union)]
pub enum TelegramChatResponse {
    Response(TelegramChatConnector),
}

pub fn chat_ids(ctx: &Context<'_>) -> Result<TelegramChatResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
        },
    )?;

    // let connection_manager = ctx.get_connection_manager();
    // let items = get_telegram_chats(
    //     connection_manager,
    //     page.map(PaginationOption::from),
    //     filter.map(|filter| filter.to_domain()),
    //     // Currently only one sort option is supported, use the first from the list.
    //     sort.and_then(|mut sort_list| sort_list.pop())
    //         .map(|sort| sort.to_domain()),
    // )
    // .map_err(StandardGraphqlError::from_list_error)?;

    // Ok(TelegramChatResponse::Response(
    //     TelegramChatConnector::from_domain(vec![]),
    // ))
    Err(StandardGraphqlError::InternalError("Not implemented".to_string()).into())
}
