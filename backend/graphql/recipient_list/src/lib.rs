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
use repository::RecipientListFilter;
use service::auth::{Resource, ResourceAccessRequest};

#[derive(Default, Clone)]
pub struct RecipientListQueries;

#[Object]
impl RecipientListQueries {
    /// Query "recipient_list" entries
    pub async fn recipient_lists(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<RecipientListFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<RecipientListSortInput>>,
    ) -> Result<RecipientListsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::QueryRecipientLists,
            },
        )?;

        let service_context = ctx.service_context(Some(&user))?;

        let recipient_lists = service_context
            .service_provider
            .recipient_list_service
            .get_recipient_lists(
                &service_context,
                page.map(PaginationOption::from),
                filter.map(RecipientListFilter::from),
                // Currently only one sort option is supported, use the first from the list.
                sort.and_then(|mut sort_list| sort_list.pop())
                    .map(|sort| sort.to_domain()),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(RecipientListsResponse::Response(
            RecipientListConnector::from_domain(recipient_lists),
        ))
    }
}

#[derive(Default, Clone)]
pub struct RecipientListMutations;

#[Object]
impl RecipientListMutations {
    async fn create_recipient_list(
        &self,
        ctx: &Context<'_>,
        input: CreateRecipientListInput,
    ) -> Result<CreateRecipientListResponse> {
        create_recipient_list(ctx, input)
    }

    async fn update_recipient_list(
        &self,
        ctx: &Context<'_>,
        input: UpdateRecipientListInput,
    ) -> Result<UpdateRecipientListResponse> {
        update_recipient_list(ctx, input)
    }

    async fn delete_recipient_list(
        &self,
        ctx: &Context<'_>,
        recipient_list_id: String,
    ) -> Result<DeleteRecipientListResponse> {
        delete_recipient_list(ctx, &recipient_list_id)
    }
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use graphql_core::assert_graphql_query;
    use graphql_core::test_helpers::setup_graphql_test;
    use repository::{mock::MockDataInserts, StorageConnectionManager};
    use repository::{
        EqualFilter, PaginationOption, RecipientList, RecipientListFilter, RecipientListSort,
        RecipientListSortField, Sort,
    };
    use serde_json::json;

    use service::recipient_list::RecipientListServiceTrait;
    use service::test_utils::get_test_settings;
    use service::{
        service_provider::{ServiceContext, ServiceProvider},
        ListError, ListResult,
    };

    use crate::RecipientListQueries;

    type GetRecipientLists = dyn Fn(
            Option<PaginationOption>,
            Option<RecipientListFilter>,
            Option<RecipientListSort>,
        ) -> Result<ListResult<RecipientList>, ListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<GetRecipientLists>);

    impl RecipientListServiceTrait for TestService {
        fn get_recipient_lists(
            &self,
            _ctx: &ServiceContext,
            pagination: Option<PaginationOption>,
            filter: Option<RecipientListFilter>,
            sort: Option<RecipientListSort>,
        ) -> Result<ListResult<RecipientList>, ListError> {
            (self.0)(pagination, filter, sort)
        }
    }

    pub fn service_provider(
        recipient_list_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider =
            ServiceProvider::new(connection_manager.clone(), get_test_settings(""));
        service_provider.recipient_list_service = Box::new(recipient_list_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_recipient_lists_success() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            RecipientListQueries,
            EmptyMutation,
            "test_graphql_recipient_lists_success",
            MockDataInserts::none().recipient_lists(),
        )
        .await;

        let query = r#"
        query {
            recipientLists {
              ... on RecipientListConnector {
                nodes {
                  id
                  name
                  description
                }
                totalCount
              }
            }
        }
        "#;

        // Test single record
        let test_service = TestService(Box::new(|_, _, _| {
            Ok(ListResult {
                rows: vec![RecipientList {
                    id: "test_id".to_string(),
                    name: "test_name".to_string(),
                    description: "description".to_string(),
                }],
                count: 1,
            })
        }));

        let expected = json!({
              "recipientLists": {
                  "nodes": [
                      {
                          "id": "test_id",
                          "name": "test_name",
                          "description": "description",
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
              "recipientLists": {
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
    async fn test_graphql_recipient_lists_inputs() {
        let (_, _, connection_manager, settings) = setup_graphql_test(
            RecipientListQueries,
            EmptyMutation,
            "test_graphql_recipient_list_inputs",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"
        query(
            $sort: [RecipientListSortInput]
            $filter: RecipientListFilterInput
          ) {
            recipientLists(sort: $sort, filter: $filter) {
              __typename
            }
          }

        "#;

        let expected = json!({
              "recipientLists": {
                  "__typename": "RecipientListConnector"
              }
          }
        );

        // Test sort by username no desc
        let test_service = TestService(Box::new(|_, _, sort| {
            assert_eq!(
                sort,
                Some(Sort {
                    key: RecipientListSortField::Name,
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
                    key: RecipientListSortField::Name,
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
                Some(RecipientListFilter::new().id(EqualFilter::equal_to("match_id")))
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
