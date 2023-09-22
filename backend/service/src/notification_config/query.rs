use chrono::NaiveDateTime;
use repository::{
    EqualFilter, NotificationConfigFilter, NotificationConfigKind, NotificationConfigRepository,
    NotificationConfigRowRepository, NotificationConfigSort, PaginationOption,
};
use util::i64_to_u32;

use crate::{
    get_default_pagination, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

use super::NotificationConfig;

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

pub fn get_notification_configs(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<NotificationConfigFilter>,
    sort: Option<NotificationConfigSort>,
) -> Result<ListResult<NotificationConfig>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = NotificationConfigRepository::new(&ctx.connection);

    Ok(ListResult {
        rows: repository.query(pagination, filter.clone(), sort)?,
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_notification_config(
    ctx: &ServiceContext,
    id: String,
) -> Result<NotificationConfig, SingleRecordError> {
    let repository = NotificationConfigRepository::new(&ctx.connection);

    let mut result = repository
        .query_by_filter(NotificationConfigFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record)
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}

pub fn get_notification_configs_by_kind_and_next_check_date(
    ctx: &ServiceContext,
    kind: NotificationConfigKind,
    datetime: NaiveDateTime,
) -> Result<Vec<NotificationConfig>, ListError> {
    let repository = NotificationConfigRowRepository::new(&ctx.connection);
    let result = repository.find_all_due_by_kind(kind, datetime)?;

    Ok(result)
}
