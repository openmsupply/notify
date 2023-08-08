pub mod mutations;
mod queries;
pub mod types;
pub use mutations::*;

use async_graphql::*;

#[derive(Default, Clone)]
pub struct TelegramQueries;

#[Object]
impl TelegramQueries {
    #[allow(non_snake_case)]
    pub async fn apiVersion(&self) -> String {
        "0.1".to_string()
    }
}
