use repository::{
    EqualFilter, PaginationOption, RecipientListFilter, RecipientListRepository, RecipientListSort,
};

use crate::{
    get_default_pagination, i64_to_u32, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

use super::RecipientList;

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_recipient_lists(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<RecipientListFilter>,
    sort: Option<RecipientListSort>,
) -> Result<ListResult<RecipientList>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = RecipientListRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_recipient_list(
    ctx: &ServiceContext,
    id: String,
) -> Result<RecipientList, SingleRecordError> {
    let repository = RecipientListRepository::new(&ctx.connection);

    let mut result =
        repository.query_by_filter(RecipientListFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}
