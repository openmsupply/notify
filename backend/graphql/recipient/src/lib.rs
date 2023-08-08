mod mutations;
use self::mutations::*;
mod types;
use self::types::*;

use async_graphql::*;
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::*;
use repository::PaginationOption;
use repository::RecipientFilter;
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct RecipientQueries;

#[Object]
impl RecipientQueries {
    /// Query "recipient" entries
    pub async fn recipients(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<RecipientFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<RecipientSortInput>>,
    ) -> Result<RecipientsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::QueryRecipients,
            },
        )?;

        let service_context = ctx.service_context(Some(&user))?;

        let recipients = service_context
            .service_provider
            .recipient_service
            .get_recipients(
                &service_context,
                page.map(PaginationOption::from),
                filter.map(RecipientFilter::from),
                // Currently only one sort option is supported, use the first from the list.
                sort.and_then(|mut sort_list| sort_list.pop())
                    .map(|sort| sort.to_domain()),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(RecipientsResponse::Response(
            RecipientConnector::from_domain(recipients),
        ))
    }
}

#[derive(Default, Clone)]
pub struct RecipientMutations;

#[Object]
impl RecipientMutations {
    async fn create_recipient(
        &self,
        ctx: &Context<'_>,
        input: CreateRecipientInput,
    ) -> Result<CreateRecipientResponse> {
        create_recipient(ctx, input)
    }

    async fn update_recipient(
        &self,
        ctx: &Context<'_>,
        input: UpdateRecipientInput,
    ) -> Result<UpdateRecipientResponse> {
        update_recipient(ctx, input)
    }

    async fn delete_recipient(
        &self,
        ctx: &Context<'_>,
        recipient_id: String,
    ) -> Result<DeleteRecipientResponse> {
        delete_recipient(ctx, &recipient_id)
    }
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use graphql_core::assert_graphql_query;
    use graphql_core::test_helpers::setup_graphql_test;
    use repository::{mock::MockDataInserts, StorageConnectionManager};
    use repository::{
        EqualFilter, NotificationType, PaginationOption, Recipient, RecipientFilter, RecipientSort,
        RecipientSortField, Sort,
    };
    use serde_json::json;

    use service::recipient::RecipientServiceTrait;
    use service::test_utils::get_test_settings;
    use service::{
        service_provider::{ServiceContext, ServiceProvider},
        ListError, ListResult,
    };

    use crate::RecipientQueries;

    type GetRecipients = dyn Fn(
            Option<PaginationOption>,
            Option<RecipientFilter>,
            Option<RecipientSort>,
        ) -> Result<ListResult<Recipient>, ListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<GetRecipients>);

    impl RecipientServiceTrait for TestService {
        fn get_recipients(
            &self,
            _ctx: &ServiceContext,
            pagination: Option<PaginationOption>,
            filter: Option<RecipientFilter>,
            sort: Option<RecipientSort>,
        ) -> Result<ListResult<Recipient>, ListError> {
            (self.0)(pagination, filter, sort)
        }
    }

    pub fn service_provider(
        recipient_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider =
            ServiceProvider::new(connection_manager.clone(), get_test_settings(""));
        service_provider.recipient_service = Box::new(recipient_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_recipients_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            RecipientQueries,
            EmptyMutation,
            "test_graphql_recipients_success",
            MockDataInserts::none().recipients(),
        )
        .await;

        let query = r#"
        query {
            recipients {
              ... on RecipientConnector {
                nodes {
                  id
                  name
                  toAddress
                  notificationType
                }
                totalCount
              }
            }
        }
        "#;

        // Test single record
        let test_service = TestService(Box::new(|_, _, _| {
            Ok(ListResult {
                rows: vec![Recipient {
                    id: "test_id".to_string(),
                    name: "test_name".to_string(),
                    to_address: "email@x.com".to_string(),
                    notification_type: NotificationType::Email,
                }],
                count: 1,
            })
        }));

        let expected = json!({
              "recipients": {
                  "nodes": [
                      {
                          "id": "test_id",
                          "name": "test_name",
                          "toAddress": "email@x.com",
                          "notificationType": "EMAIL",
                      },
                  ],
                  "totalCount": 1
              }
          }
        );

        assert_graphql_query!(
            &settings,
            query,
            &None,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );

        // Test no records

        let test_service = TestService(Box::new(|_, _, _| {
            Ok(ListResult {
                rows: Vec::new(),
                count: 0,
            })
        }));

        let expected = json!({
              "recipients": {
                  "nodes": [

                  ],
                  "totalCount": 0
              }
          }
        );

        assert_graphql_query!(
            &settings,
            query,
            &None,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }

    #[actix_rt::test]
    async fn test_graphql_recipients_inputs() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            RecipientQueries,
            EmptyMutation,
            "test_graphql_recipient_inputs",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"
        query(
            $sort: [RecipientSortInput]
            $filter: RecipientFilterInput
          ) {
            recipients(sort: $sort, filter: $filter) {
              __typename
            }
          }

        "#;

        let expected = json!({
              "recipients": {
                  "__typename": "RecipientConnector"
              }
          }
        );

        // Test sort by username no desc
        let test_service = TestService(Box::new(|_, _, sort| {
            assert_eq!(
                sort,
                Some(Sort {
                    key: RecipientSortField::Name,
                    desc: None
                })
            );
            Ok(ListResult::empty())
        }));

        let variables = json!({
          "sort": [{
            "key": "name",
          }]
        });

        assert_graphql_query!(
            &settings,
            query,
            &Some(variables),
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );

        // Test sort by username with desc
        let test_service = TestService(Box::new(|_, _, sort| {
            assert_eq!(
                sort,
                Some(Sort {
                    key: RecipientSortField::Name,
                    desc: Some(true)
                })
            );
            Ok(ListResult::empty())
        }));

        let variables = json!({
          "sort": [{
            "key": "name",
            "desc": true
          }]
        });

        assert_graphql_query!(
            &settings,
            query,
            &Some(variables),
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );

        // Test filter
        let test_service = TestService(Box::new(|_, filter, _| {
            assert_eq!(
                filter,
                Some(RecipientFilter::new().id(EqualFilter::equal_to("match_id")))
            );
            Ok(ListResult::empty())
        }));

        let variables = json!({
          "filter": {
            "id": { "equalTo": "match_id"},
          }
        });

        assert_graphql_query!(
            &settings,
            query,
            &Some(variables),
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }
}
