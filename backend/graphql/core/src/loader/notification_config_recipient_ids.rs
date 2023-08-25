use repository::{EqualFilter, NotificationConfigRecipientFilter, Pagination};
use repository::{NotificationConfigRecipientRepository, StorageConnectionManager};

use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct NotificationConfigRecipientIdsLoader {
    pub connection_manager: StorageConnectionManager,
}

#[async_trait::async_trait]
impl Loader<String> for NotificationConfigRecipientIdsLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        notification_config_ids: &[String],
    ) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let connection = self.connection_manager.connection()?;

        let config_recipient_repo = NotificationConfigRecipientRepository::new(&connection);

        let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

        for notification_config_id in notification_config_ids {
            let recipient_ids = config_recipient_repo
                .query(
                    Pagination::all(),
                    Some(
                        NotificationConfigRecipientFilter::new()
                            .notification_config_id(EqualFilter::equal_to(notification_config_id)),
                    ),
                )?
                .into_iter()
                .map(|config_recipient| config_recipient.recipient_id)
                .collect();

            result_map.insert(notification_config_id.to_string(), recipient_ids);
        }
        Ok(result_map)
    }
}
