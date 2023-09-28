use repository::{
    EqualFilter, NotificationQueryFilter, NotificationQueryRepository, NotificationQuerySort,
    PaginationOption,
};
use util::number_conversions::i64_to_u32;

use crate::{
    get_default_pagination, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

use super::NotificationQuery;

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_notification_queries(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<NotificationQueryFilter>,
    sort: Option<NotificationQuerySort>,
) -> Result<ListResult<NotificationQuery>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = NotificationQueryRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_notification_query(
    ctx: &ServiceContext,
    id: String,
) -> Result<NotificationQuery, SingleRecordError> {
    let repository = NotificationQueryRepository::new(&ctx.connection);

    let mut result = repository
        .query_by_filter(NotificationQueryFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}
