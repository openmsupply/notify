use async_graphql::{Object, SimpleObject, Union};
use graphql_core::simple_generic_errors::NodeError;

use repository::NotificationQuery;
use service::ListResult;
use util::usize_to_u32;

#[derive(Union)]
pub enum NotificationQuerysResponse {
    Response(NotificationQueryConnector),
}

#[derive(Union)]
pub enum NotificationQueryResponse {
    Error(NodeError),
    Response(NotificationQueryNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct NotificationQueryNode {
    pub recipient_list: NotificationQuery,
}

#[Object]
impl NotificationQueryNode {
    pub async fn id(&self) -> &str {
        &self.row().id
    }
    pub async fn name(&self) -> &str {
        &self.row().name
    }
    pub async fn description(&self) -> &str {
        &self.row().description
    }
    pub async fn query(&self) -> &str {
        &self.row().query
    }
    pub async fn required_parameters(&self) -> Result<Vec<String>, async_graphql::Error> {
        // Convert the parameters from a JSON array to a Vec<String>
        let parameters = serde_json::from_str::<Vec<String>>(&self.row().required_parameters)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(parameters)
    }
}

impl NotificationQueryNode {
    pub fn from_domain(recipient_list: NotificationQuery) -> NotificationQueryNode {
        NotificationQueryNode { recipient_list }
    }

    pub fn row(&self) -> &NotificationQuery {
        &self.recipient_list
    }
}

#[derive(SimpleObject)]
pub struct NotificationQueryConnector {
    total_count: u32,
    nodes: Vec<NotificationQueryNode>,
}

impl NotificationQueryConnector {
    pub fn from_domain(
        recipient_lists: ListResult<NotificationQuery>,
    ) -> NotificationQueryConnector {
        NotificationQueryConnector {
            total_count: recipient_lists.count,
            nodes: recipient_lists
                .rows
                .into_iter()
                .map(NotificationQueryNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(recipient_lists: Vec<NotificationQuery>) -> NotificationQueryConnector {
        NotificationQueryConnector {
            total_count: usize_to_u32(recipient_lists.len()),
            nodes: recipient_lists
                .into_iter()
                .map(NotificationQueryNode::from_domain)
                .collect(),
        }
    }
}
