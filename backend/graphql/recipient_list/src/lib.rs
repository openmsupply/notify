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
use repository::PaginationOption;
use repository::RecipientListFilter;
use repository::SqlRecipientListFilter;
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
                resource: Resource::ServerAdmin,
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

    /// Query "sql_recipient_list" entries
    pub async fn sql_recipient_lists(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Pagination option (first and offset)")] page: Option<PaginationInput>,
        #[graphql(desc = "Filter option")] filter: Option<RecipientListFilterInput>,
        #[graphql(desc = "Sort options (only first sort input is evaluated for this endpoint)")]
        sort: Option<Vec<RecipientListSortInput>>,
    ) -> Result<SqlRecipientListsResponse> {
        let user = validate_auth(
            ctx,
            &ResourceAccessRequest {
                resource: Resource::ServerAdmin,
            },
        )?;

        let service_context = ctx.service_context(Some(&user))?;

        let recipient_lists = service_context
            .service_provider
            .sql_recipient_list_service
            .get_sql_recipient_lists(
                &service_context,
                page.map(PaginationOption::from),
                filter.map(SqlRecipientListFilter::from),
                // Currently only one sort option is supported, use the first from the list.
                sort.and_then(|mut sort_list| sort_list.pop())
                    .map(|sort| sort.to_domain_sql()),
            )
            .map_err(StandardGraphqlError::from_list_error)?;

        Ok(SqlRecipientListsResponse::Response(
            SqlRecipientListConnector::from_domain(recipient_lists),
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
    ) -> Result<ModifyRecipientListResponse> {
        create_recipient_list(ctx, input)
    }

    async fn update_recipient_list(
        &self,
        ctx: &Context<'_>,
        input: UpdateRecipientListInput,
    ) -> Result<ModifyRecipientListResponse> {
        update_recipient_list(ctx, input)
    }

    async fn delete_recipient_list(
        &self,
        ctx: &Context<'_>,
        recipient_list_id: String,
    ) -> Result<DeleteRecipientListResponse> {
        delete_recipient_list(ctx, &recipient_list_id)
    }

    async fn add_recipient_to_list(
        &self,
        ctx: &Context<'_>,
        input: AddRecipientToListInput,
    ) -> Result<ModifyRecipientListMembersResponse> {
        add_recipient_to_list(ctx, input)
    }

    async fn remove_recipient_from_list(
        &self,
        ctx: &Context<'_>,
        input: RemoveRecipientFromListInput,
    ) -> Result<ModifyRecipientListMembersResponse> {
        remove_recipient_from_list(ctx, input)
    }

    async fn create_sql_recipient_list(
        &self,
        ctx: &Context<'_>,
        input: CreateSqlRecipientListInput,
    ) -> Result<ModifySqlRecipientListResponse> {
        create_sql_recipient_list(ctx, input)
    }

    async fn update_sql_recipient_list(
        &self,
        ctx: &Context<'_>,
        input: UpdateSqlRecipientListInput,
    ) -> Result<ModifySqlRecipientListResponse> {
        update_sql_recipient_list(ctx, input)
    }

    async fn delete_sql_recipient_list(
        &self,
        ctx: &Context<'_>,
        sql_recipient_list_id: String,
    ) -> Result<DeleteSqlRecipientListResponse> {
        delete_sql_recipient_list(ctx, &sql_recipient_list_id)
    }
}
