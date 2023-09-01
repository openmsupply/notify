use repository::{
    EqualFilter, PaginationOption, SqlRecipientListFilter, SqlRecipientListRepository, SqlRecipientListSort,
};
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
