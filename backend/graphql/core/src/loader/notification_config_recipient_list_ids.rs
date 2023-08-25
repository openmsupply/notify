use repository::{EqualFilter, NotificationConfigRecipientListFilter, Pagination};
use repository::{NotificationConfigRecipientListRepository, StorageConnectionManager};

use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct NotificationConfigRecipientListIdsLoader {
    pub connection_manager: StorageConnectionManager,
}

#[async_trait::async_trait]
impl Loader<String> for NotificationConfigRecipientListIdsLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        notification_config_ids: &[String],
    ) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let connection = self.connection_manager.connection()?;

        let config_recipient_list_repo =
            NotificationConfigRecipientListRepository::new(&connection);

        let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

        for notification_config_id in notification_config_ids {
            let recipient_list_ids = config_recipient_list_repo
                .query(
                    Pagination::all(),
                    Some(
                        NotificationConfigRecipientListFilter::new()
                            .notification_config_id(EqualFilter::equal_to(notification_config_id)),
                    ),
                )?
                .into_iter()
                .map(|config_recipient_list| config_recipient_list.recipient_list_id)
                .collect();

            result_map.insert(notification_config_id.to_string(), recipient_list_ids);
        }
        Ok(result_map)
    }
}
