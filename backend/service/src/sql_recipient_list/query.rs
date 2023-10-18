use crate::sql_recipient_list::BasicRecipientRow;
use repository::{
    EqualFilter, PaginationOption, SqlRecipientListFilter, SqlRecipientListRepository,
    SqlRecipientListRowRepository, SqlRecipientListSort,
};
use tera::{Context, Tera};
use util::number_conversions::i64_to_u32;

use crate::{
    get_default_pagination, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

use super::SqlRecipientList;

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_sql_recipient_lists(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<SqlRecipientListFilter>,
    sort: Option<SqlRecipientListSort>,
) -> Result<ListResult<SqlRecipientList>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = SqlRecipientListRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_sql_recipient_list(
    ctx: &ServiceContext,
    id: String,
) -> Result<SqlRecipientList, SingleRecordError> {
    let repository = SqlRecipientListRepository::new(&ctx.connection);

    let mut result =
        repository.query_by_filter(SqlRecipientListFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}

pub fn get_sql_recipients(
    ctx: &ServiceContext,
    sql_recipient_list_id: String,
    params: serde_json::Value,
) -> Result<ListResult<BasicRecipientRow>, ListError> {
    let repository = SqlRecipientListRowRepository::new(&ctx.connection);

    let sql_list = repository.find_one_by_id(&sql_recipient_list_id)?;
    let sql_list = match sql_list {
        Some(sql_list) => sql_list,
        None => {
            return Err(ListError::InvalidRequest(format!(
                "No sql recipient list found with id: {}",
                sql_recipient_list_id
            )))
        }
    };

    get_sql_recipients_by_sql_query(ctx, sql_list.query, params)
}

pub fn get_sql_recipients_by_sql_query(
    ctx: &ServiceContext,
    query: String,
    params: serde_json::Value,
) -> Result<ListResult<BasicRecipientRow>, ListError> {
    // Pass params to template to get the full query
    let tera_context = Context::from_value(params).map_err(|e| {
        ListError::InvalidRequest(format!(
            "Failed to convert params to tera context: {}",
            e.to_string()
        ))
    })?;

    let full_query = Tera::one_off(&query, &tera_context, false).map_err(|e| {
        ListError::InvalidRequest(format!(
            "Failed to parse query as tera template: {}",
            e.to_string()
        ))
    })?;

    // query the datasource with the templated query
    let result = ctx
        .service_provider
        .datasource_service
        .run_recipient_query(full_query)
        .map_err(|e| {
            ListError::InvalidRequest(format!("Failed to run query on datasource: {:?}", e))
        })?;

    Ok(ListResult {
        count: result.len() as u32,
        rows: result,
    })
}
