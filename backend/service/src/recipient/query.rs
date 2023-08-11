use repository::{
    EqualFilter, PaginationOption, RecipientFilter, RecipientRepository, RecipientSort,
};
use util::i64_to_u32;

use crate::{
    get_default_pagination, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

use super::Recipient;

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_recipients(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<RecipientFilter>,
    sort: Option<RecipientSort>,
) -> Result<ListResult<Recipient>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = RecipientRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_recipient(ctx: &ServiceContext, id: String) -> Result<Recipient, SingleRecordError> {
    let repository = RecipientRepository::new(&ctx.connection);

    let mut result =
        repository.query_by_filter(RecipientFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}
