use repository::{EqualFilter, NotificationConfigFilter, NotificationConfigRepository, Pagination};
use repository::{NotificationConfigRow, StorageConnectionManager};

use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct NotificationConfigLoader {
    pub connection_manager: StorageConnectionManager,
}

#[async_trait::async_trait]
impl Loader<String> for NotificationConfigLoader {
    type Value = NotificationConfigRow;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        config_ids: &[String],
    ) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let connection = self.connection_manager.connection()?;
        let repo = NotificationConfigRepository::new(&connection);
        Ok(repo
            .query(
                Pagination::all(),
                Some(
                    NotificationConfigFilter::new().id(EqualFilter::equal_any(config_ids.to_vec())),
                ),
                None,
            )?
            .into_iter()
            .map(|config| (config.id.clone(), config))
            .collect())
    }
}
