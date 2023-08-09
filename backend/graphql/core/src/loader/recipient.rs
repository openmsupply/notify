use repository::{
    EqualFilter, Pagination, Recipient, RecipientFilter, RecipientListMemberRepository,
    RecipientRepository,
};
use repository::{RecipientListMemberFilter, StorageConnectionManager};

use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct RecipientLoader {
    pub connection_manager: StorageConnectionManager,
}

#[async_trait::async_trait]
impl Loader<String> for RecipientLoader {
    type Value = Vec<Recipient>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        recipient_list_ids: &[String],
    ) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let connection = self.connection_manager.connection()?;

        let member_repo = RecipientListMemberRepository::new(&connection);
        let recipient_repo = RecipientRepository::new(&connection);

        let mut result_map: HashMap<String, Vec<Recipient>> = HashMap::new();

        for recipient_list_id in recipient_list_ids {
            let recipient_ids = member_repo
                .query(
                    Pagination::all(),
                    Some(
                        RecipientListMemberFilter::new()
                            .recipient_list_id(EqualFilter::equal_to(recipient_list_id)),
                    ),
                )?
                .into_iter()
                .map(|member| member.recipient_id)
                .collect();

            let recipients = recipient_repo.query(
                Pagination::all(),
                Some(RecipientFilter::new().id(EqualFilter::equal_any(recipient_ids))),
                None,
            )?;

            result_map.insert(recipient_list_id.to_string(), recipients);
        }
        Ok(result_map)
    }
}
